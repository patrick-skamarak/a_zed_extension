use zed_extension_api::{LanguageServerId, Worktree};

mod java;

pub struct ExtensionSettings<'a> {
    pub worktree: &'a Worktree,
    pub language_server_id: &'a LanguageServerId,
}

impl<'a> ExtensionSettings<'a> {
    pub fn new(worktree: &'a Worktree, language_server_id: &'a LanguageServerId) -> Self {
        ExtensionSettings {
            worktree,
            language_server_id,
        }
    }
}
