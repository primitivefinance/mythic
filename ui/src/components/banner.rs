//! A simple banner image that can be rendered on startup, closing, or loading
//! screens.

use iced::{
    widget::{container, image, Container},
    Length,
};

/// Returns a banner image with the given width.
pub fn banner<'a, Message>(width: u16) -> Container<'a, Message> {
    container(
        // This should go away once we unify resource loading on native
        // platforms
        if cfg!(target_arch = "wasm32") {
            image("../images/banner.jpg")
        } else {
            image(format!("{}/images/banner.jpg", env!("CARGO_MANIFEST_DIR")))
        }
        .width(width),
    )
    .width(Length::Fill)
    .center_x()
}
