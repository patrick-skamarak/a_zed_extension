pub static LANGUAGE_SERVER_NAME: &'static str = "apex-jorje-lsp";
pub static LANGUAGE_NAME: &'static str = "Apex";

pub static JAVA_MAIN: &'static str = "apex.jorje.lsp.ApexLanguageServerLauncher";
pub static JAVA_FIRST_ARG: &'static str = "-cp";

pub static LSP_JAR_PATH: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bin/lsp/apex-jorje-lsp.jar",
);
