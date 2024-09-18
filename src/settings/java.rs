use super::ExtensionSettings;
use crate::constants::{self, JAVA_VERSION_REGEX_PATTERN};
use regex::Regex;
use std::process::Command;
use std::sync::LazyLock;

pub static REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(JAVA_VERSION_REGEX_PATTERN).expect("failed to compile regex"));

impl<'a> ExtensionSettings<'a> {
    pub fn get_java_path(&self) -> Option<String> {
        let java_path_setting = self
            .get_lsp_settings()
            .unwrap()
            .settings
            .unwrap()
            .as_object()?
            .get(constants::JAVA_PATH_JSON_TAG);

        Some(String::from(
            "/Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home/bin/java",
        ))
    }
}
