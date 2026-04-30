use std::vec;

use zed_extension_api as zed;

struct Zizmor {}

impl zed::Extension for Zizmor {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let Some(zizmor) = worktree.which("zizmor") else {
            return Err(
                "zizmor not found in PATH. Please install it to use the zizmor extension.".into(),
            );
        };

        Ok(zed::Command {
            command: zizmor,
            args: vec!["--lsp".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(Zizmor);
