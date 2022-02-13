use gst::glib;
use gst::prelude::*;

mod imp;

glib::wrapper! {
    pub struct DkcDummySource(ObjectSubclass<imp::DkcDummySource>) @extends gst::Bin, gst::Element, gst::Object;
}

// GStreamer elements need to be thread-safe. For the private implementation this is automatically
// enforced but for the public wrapper type we need to specify this manually.
unsafe impl Send for DkcDummySource {}
unsafe impl Sync for DkcDummySource {}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "dkcdummysource",
        gst::Rank::None,
        DkcDummySource::static_type(),
    )
}
