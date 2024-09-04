use super::ExtensionSettings;

impl ExtensionSettings<'_> {
    pub fn get_java_path(&self) -> Option<String> {
        Some(String::from(
            "/Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home/bin/java",
        ))
    }
}
