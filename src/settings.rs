use std::error::Error;

use crate::constants;
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
    pub fn get_lsp_settings(&self) -> Result<zed_extension_api::settings::LspSettings, String> {
        zed_extension_api::settings::LspSettings::for_worktree(
            &constants::LANGUAGE_SERVER_NAME,
            self.worktree,
        )
    }
    pub fn get_language_settings(
        &self,
    ) -> Result<zed_extension_api::settings::LanguageSettings, String> {
        zed_extension_api::settings::LanguageSettings::for_worktree(
            Some(constants::LANGUAGE_NAME),
            &self.worktree,
        )
    }
}
