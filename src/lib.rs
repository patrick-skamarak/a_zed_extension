mod constants;
mod settings;

use constants::{JAVA_FIRST_ARG, JAVA_MAIN, LSP_JAR_PATH};
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
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let extension_settings = settings::ExtensionSettings::new(worktree, language_server_id);
        println!("{:?}", extension_settings.get_lsp_settings());
        println!("{:?}", extension_settings.get_language_settings());
        println!("{:?}", language_server_id);
        let mut args: Vec<String> = Vec::new();
        args.push(String::from(JAVA_FIRST_ARG));
        args.push(String::from(LSP_JAR_PATH));
        args.push(String::from(JAVA_MAIN));
        zed::Result::Ok(zed::Command {
            command: extension_settings.get_java_path().unwrap(),
            args,
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(ApexExtension);
