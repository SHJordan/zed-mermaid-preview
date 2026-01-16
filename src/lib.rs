use zed_extension_api::{self as zed, LanguageServerId, Result};

struct MermaidPreviewExtension {
    cached_binary_path: Option<String>,
}

impl MermaidPreviewExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        if let Ok(path) = std::env::var("MERMAID_LSP_PATH") {
            if std::fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                return Ok(path);
            }
        }

        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let package_name = "@my-org/my-agent-lsp";
        let latest_version = zed::npm_package_latest_version(package_name)?;

        let repository_url = "node_modules/@my-org/my-agent-lsp/dist/server.js";
        if !std::fs::metadata(repository_url).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::npm_install_package(package_name, &latest_version)?;
        }

        self.cached_binary_path = Some(repository_url.to_string());
        Ok(repository_url.to_string())
    }
}

impl zed::Extension for MermaidPreviewExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.language_server_binary_path(language_server_id)?;
        let node_binary = zed::node_binary_path()?;

        Ok(zed::Command {
            command: node_binary,
            args: vec![server_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed_extension_api::register_extension!(MermaidPreviewExtension);
