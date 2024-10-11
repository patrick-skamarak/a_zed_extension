use std::ops::Deref;

use zed_extension_api::{
    serde_json::{Map, Value},
    settings::LspSettings,
    LanguageServerId, Worktree,
};

mod java;

pub struct ExtensionSettings<'a> {
    worktree: &'a Worktree,
    language_server_id: &'a LanguageServerId,
    language_server_name: &'a str,
    language_name: &'a str,
}

struct Wrapper<T>(T);

impl Deref for Wrapper<LspSettings> {
    type Target = LspSettings;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Wrapper<LspSettings> {}

// pub fn get_value(
//     json_object: Map<String, Value>,
//     list: Vec<&str>,
// ) -> Result<Option<Value>, String> {
//     if (list.len() == 0) {
//         return Result::Ok(Option::None);
//     }
//     if (list.len() == 1) {
//         return json_object;
//     }
//     todo!()
// }

impl<'a> ExtensionSettings<'a> {
    pub fn new(
        worktree: &'a Worktree,
        language_server_id: &'a LanguageServerId,
        language_server_name: &'a str,
        language_name: &'a str,
    ) -> Self {
        Wrapper(
            zed_extension_api::settings::LspSettings::for_worktree(language_server_name, worktree)
                .unwrap(),
        )
        .test();
        ExtensionSettings {
            worktree,
            language_server_id,
            language_server_name,
            language_name,
        }
    }
}
