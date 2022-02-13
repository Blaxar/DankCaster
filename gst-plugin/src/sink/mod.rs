use gst::glib;
use gst::prelude::*;

mod imp;

glib::wrapper! {
    pub struct DkcDummySink(ObjectSubclass<imp::DkcDummySink>) @extends gst::Bin, gst::Element, gst::Object;
}

// GStreamer elements need to be thread-safe. For the private implementation this is automatically
// enforced but for the public wrapper type we need to specify this manually.
unsafe impl Send for DkcDummySink {}
unsafe impl Sync for DkcDummySink {}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "dkcdummysink",
        gst::Rank::None,
        DkcDummySink::static_type(),
    )
}
