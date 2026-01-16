use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId, Os, Result,
};

const GITHUB_REPOSITORY: &str = "dawsh2/zed-mermaid-preview";
const CACHE_ROOT: &str = "mermaid-lsp-cache";

struct MermaidPreviewExtension {
    lsp_path: Option<String>,
}

impl MermaidPreviewExtension {
    fn lsp_binary_name() -> &'static str {
        if cfg!(target_os = "windows") {
            "mermaid-lsp.exe"
        } else {
            "mermaid-lsp"
        }
    }

    fn find_in_path(binary_name: &str) -> Option<PathBuf> {
        let command = if cfg!(target_os = "windows") {
            "where"
        } else {
            "which"
        };

        if let Ok(output) = Command::new(command).arg(binary_name).output() {
            if output.status.success() {
                let path_string = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = path_string.lines().next() {
                    let path_str = first_line.trim();
                    if !path_str.is_empty() {
                        return Some(PathBuf::from(path_str));
                    }
                }
            }
        }
        None
    }

    fn candidate_paths(extension_dir: &Path, binary_name: &str) -> Vec<PathBuf> {
        let mut candidates = vec![extension_dir.join(binary_name)];

        let target = extension_dir.join("target");
        candidates.push(target.join("release").join(binary_name));
        candidates.push(target.join("debug").join(binary_name));
        candidates.push(extension_dir.join("bin").join(binary_name));

        if Path::new("lsp/Cargo.toml").exists() {
            candidates.push(extension_dir.join("lsp/target/release").join(binary_name));
        }

        if let Ok(current_dir) = env::current_dir() {
            let cache_root = current_dir.join(CACHE_ROOT);
            if let Ok(entries) = fs::read_dir(cache_root) {
                for entry in entries.flatten() {
                    candidates.push(entry.path().join(binary_name));
                }
            }
        }

        candidates
    }

    fn finalize_path(
        language_server_id: &LanguageServerId,
        path: PathBuf,
        cache: &mut Option<String>,
    ) -> Result<String> {
        let resolved = path
            .canonicalize()
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        *cache = Some(resolved.clone());

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(resolved)
    }

    fn match_asset(release: &zed::GithubRelease) -> Result<zed::GithubReleaseAsset> {
        let (os, arch) = zed::current_platform();
        let arch_str = match arch {
            Architecture::Aarch64 => "aarch64",
            Architecture::X86 => "x86",
            Architecture::X8664 => "x86_64",
        };

        let os_str = match os {
            Os::Mac => "apple-darwin",
            Os::Linux => "unknown-linux-gnu",
            Os::Windows => "pc-windows-msvc",
        };

        let expected_name = format!("mermaid-lsp-{arch_str}-{os_str}.zip");

        release
            .assets
            .iter()
            .find(|asset| asset.name == expected_name)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "no GitHub release asset named '{expected_name}' for platform {os:?}/{arch:?}"
                )
            })
    }

    fn download_lsp(
        &mut self,
        language_server_id: &LanguageServerId,
        binary_name: &str,
    ) -> Result<PathBuf> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            GITHUB_REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = Self::match_asset(&release)?;

        // Use current directory for cache (this is the extension's work directory)
        let work_dir =
            env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;
        let version_dir = work_dir.join(CACHE_ROOT).join(&release.version);
        let binary_path = version_dir.join(binary_name);

        if binary_path.is_file() {
            return Ok(binary_path);
        }

        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create cache directory '{version_dir:?}': {err}"))?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(
            &asset.download_url,
            &version_dir.to_string_lossy(),
            DownloadedFileType::Zip,
        )
        .map_err(|err| format!("failed to download mermaid-lsp asset: {err}"))?;

        zed::make_file_executable(&binary_path.to_string_lossy())?;

        let cache_root = work_dir.join(CACHE_ROOT);
        if let Ok(entries) = fs::read_dir(&cache_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir()
                    && path.file_name().and_then(|n| n.to_str()) != Some(&release.version)
                {
                    let _ = fs::remove_dir_all(path);
                }
            }
        }

        Ok(binary_path)
    }

    fn get_lsp_path(
        &mut self,
        language_server_id: &LanguageServerId,
        extension_dir: &Path,
    ) -> Result<String> {
        if let Some(ref path) = self.lsp_path {
            return Ok(path.clone());
        }

        if let Ok(path) = env::var("MERMAID_LSP_PATH") {
            let candidate = PathBuf::from(&path);
            if candidate.is_file() {
                return Self::finalize_path(language_server_id, candidate, &mut self.lsp_path);
            }
        }

        let lsp_binary_name = Self::lsp_binary_name();

        if let Some(path) = Self::find_in_path(lsp_binary_name) {
            return Self::finalize_path(language_server_id, path, &mut self.lsp_path);
        }

        if let Some(path) = Self::candidate_paths(extension_dir, lsp_binary_name)
            .into_iter()
            .find(|candidate| candidate.is_file())
        {
            return Self::finalize_path(language_server_id, path, &mut self.lsp_path);
        }

        match self.download_lsp(language_server_id, lsp_binary_name) {
            Ok(downloaded) if downloaded.is_file() => {
                Self::finalize_path(language_server_id, downloaded, &mut self.lsp_path)
            }
            Ok(_) => Err("Downloaded file is not a valid binary".to_string()),
            Err(e) => Err(format!("Failed to download or find LSP: {e}")),
        }
    }
}

impl zed::Extension for MermaidPreviewExtension {
    fn new() -> Self {
        Self { lsp_path: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() != "mermaid" {
            return Err(format!("Unknown language server: {language_server_id}"));
        }

        let extension_dir = PathBuf::from(worktree.root_path());
        let lsp_path = self.get_lsp_path(language_server_id, &extension_dir)?;

        Ok(zed::Command {
            command: lsp_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed_extension_api::register_extension!(MermaidPreviewExtension);
