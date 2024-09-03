use zed::serde_json::Value;
use zed_extension_api::{self as zed, LanguageServerId, Worktree};

trait Setting {
    fn new(name: &'static str) -> Self;
    fn get_value(worktree: &Worktree, language_server_id: &LanguageServerId) -> Value;
}
