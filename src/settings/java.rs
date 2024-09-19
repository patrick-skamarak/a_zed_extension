use super::ExtensionSettings;
use regex::Regex;
use std::process::Command;
use std::sync::LazyLock;

pub static JAVA_PATH_TAG: &'static str = "java_absolute_path";
pub static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"/java\.version\s*=\s*(?:11|17|21)/g").expect("failed to compile regex")
});

impl<'a> ExtensionSettings<'a> {
    pub fn get_java_path(&self) -> Option<String> {
        Some(String::from(
            "/Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home/bin/java",
        ))
    }
}
