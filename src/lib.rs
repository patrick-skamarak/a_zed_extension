use zed_extension_api::{self as zed, register_extension};

struct MyExtension;

impl zed::Extension for MyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        MyExtension {}
    }
}

register_extension!(MyExtension);
