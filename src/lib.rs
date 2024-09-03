mod constants;
mod settings;

use constants::LANGUAGE_SERVER_ID;
use zed_extension_api as zed;

struct ApexExtension;

impl zed::Extension for ApexExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        ApexExtension {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/lsp/");
        zed::Result::Ok(zed::Command {
            command: "/Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home/bin/java"
                .to_string(),
            args: vec![
                "-jar".to_string(),
                format!(
                    "{}{}{}",
                    lsp_path,
                    LANGUAGE_SERVER_ID.to_string(),
                    ".jar".to_string()
                ),
            ],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(ApexExtension);
