use zed_extension_api::{LanguageServerId, Worktree};

mod java;

trait Getter<T> {
    type Output;
    fn get(&self, _: T) -> Self::Output;
}

pub struct ExtensionSettings<'a> {
    worktree: &'a Worktree,
    language_server_id: &'a LanguageServerId,
    language_server_name: &'a str,
    language_name: &'a str,
}

impl<'a> ExtensionSettings<'a> {
    pub fn new(
        worktree: &'a Worktree,
        language_server_id: &'a LanguageServerId,
        language_server_name: &'a str,
        language_name: &'a str,
    ) -> Self {
        ExtensionSettings {
            worktree,
            language_server_id,
            language_server_name,
            language_name,
        }
    }
}
