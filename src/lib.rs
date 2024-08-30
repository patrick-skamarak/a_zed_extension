use zed_extension_api as zed;

struct MyExtension;

impl zed::Extension for MyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        MyExtension {}
    }
}

zed::register_extension!(MyExtension);
