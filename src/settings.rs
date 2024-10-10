use zed_extension_api::{
    serde_json::{Map, Value},
    settings::{LanguageSettings, LspSettings},
    LanguageServerId, Worktree,
};

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

pub fn get_value(
    json_object: Map<String, Value>,
    list: Vec<&str>,
) -> Result<Option<Value>, String> {
    if (list.len() == 0) {
        return Result::Ok(Option::None);
    }
    if (list.len() == 1) {
        return json_object;
    }
    todo!()
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
