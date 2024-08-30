use zed_extension_api as zed;

mod language_server;

pub struct MyExtension;
impl zed::Extension for MyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        MyExtension {}
    }
}

#[test]
fn extension_new_positive() {
    use zed_extension_api::Extension;
    let _ = MyExtension::new();
}
