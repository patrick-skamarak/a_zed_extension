use zed_extension_api::register_extension;

mod extension;
use extension::MyExtension;

register_extension!(MyExtension);
