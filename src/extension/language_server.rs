use std::process::{Child, Command};
// use zed_extension_api::Os;

mod apex;

pub struct LanguageServer {
    process: Child,
}

impl LanguageServer {
    fn new() -> Self {
        LanguageServer {
            process: Command::new("java")
                .arg("-jar")
                .arg("./src/bin/lsp/apex-jorje-lsp.jar")
                .spawn()
                .expect("Failed to launch child process"),
        }
    }
}

impl Drop for LanguageServer {
    fn drop(&mut self) {
        match self.process.kill() {
            Ok(()) => (),
            Err(io_err) => {
                println!(
                    "Failed to kill language server process with PID {}",
                    self.process.id()
                );
                panic!("Error: {}", io_err);
            }
        }
    }
}

#[test]
fn it_launches_and_drops() {
    use std::thread::sleep;
    use std::time::Duration;
    let _ = LanguageServer::new();
    sleep(Duration::from_secs(3));
}
