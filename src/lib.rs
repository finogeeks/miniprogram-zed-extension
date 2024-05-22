use zed_extension_api::{self as zed, Result};

struct FinclipExtension;

impl zed::Extension for FinclipExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Err("Not implemented".into())
    }
}

zed::register_extension!(FinclipExtension);
