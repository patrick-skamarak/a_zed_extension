use crate::constants;
use zed_extension_api::{
    serde_json::Value,
    settings::{self, LanguageSettings, LspSettings},
    LanguageServerId, Worktree,
};

mod java;

pub struct ExtensionSettings<'a> {
    worktree: &'a Worktree,
    language_server_id: &'a LanguageServerId,
}

impl<'a> ExtensionSettings<'a> {
    pub fn new(worktree: &'a Worktree, language_server_id: &'a LanguageServerId) -> Self {
        ExtensionSettings {
            worktree,
            language_server_id,
        }
    }
    pub fn get_lsp_settings(&self) -> Result<LspSettings, String> {
        LspSettings::for_worktree(&constants::LANGUAGE_SERVER_NAME, self.worktree)
    }
    pub fn get_language_settings(&self) -> Result<LanguageSettings, String> {
        LanguageSettings::for_worktree(Some(&constants::LANGUAGE_NAME), self.worktree)
    }
}
