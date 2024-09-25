mod constants;
mod settings;

use constants::{JAVA_FIRST_ARG, JAVA_MAIN, LSP_JAR_PATH};
use zed_extension_api::{self as zed, LanguageServerId};

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
        if (language_server_id.as_ref() != constants::LANGUAGE_SERVER_NAME) {
            return zed::Result::Err(String::from("boop"));
        }
        let extension_settings = settings::ExtensionSettings::new(
            worktree,
            language_server_id,
            constants::LANGUAGE_SERVER_NAME,
            constants::LANGUAGE_NAME,
        );
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
