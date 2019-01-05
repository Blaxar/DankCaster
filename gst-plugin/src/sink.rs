use glib;
use gst;
use gst::prelude::*;
use gst_base::prelude::*;
use gst_plugin::bin::*;

use gst_plugin::object::*;
use gst_plugin::base_src::*;
use gst_plugin::element::*;
use gobject_subclass::object::*;

struct DkcDummySinkStatic;
struct DkcDummySink {
    cat: gst::DebugCategory,
}

impl ImplTypeStatic<Bin> for DkcDummySinkStatic {
    fn get_name(&self) -> &str {
        "DkcDummySink"
    }

    fn new(&self, element: &Bin) -> Box<BinImpl<Bin>> {
        DkcDummySink::new(element)
    }

    fn class_init(&self, klass: &mut BinClass) {
        DkcDummySink::class_init(klass);
    }
}

impl ObjectImpl<Bin> for DkcDummySink {
    fn constructed(&self, bin: &Bin) {
        bin.parent_constructed();

        let video_elem = gst::ElementFactory::make("autovideosink", "testvideosink")
            .expect("Could not create video sink element.");

        let audio_elem = gst::ElementFactory::make("autoaudiosink", "testaudiosink")
            .expect("Could not create audio sink element.");

        self.add_element(bin, &video_elem);
        self.add_element(bin, &audio_elem);

        let video_pad = video_elem.get_static_pad("sink").unwrap();

        let audio_pad = audio_elem.get_static_pad("sink").unwrap();

        let video_ghost_pad = gst::GhostPad::new("video_sink", &video_pad).unwrap();

        let audio_ghost_pad = gst::GhostPad::new("audio_sink", &audio_pad).unwrap();

        bin.add_pad(&video_ghost_pad).unwrap();
        bin.add_pad(&audio_ghost_pad).unwrap();
    }
}

impl ElementImpl<Bin> for DkcDummySink {}
impl BinImpl<Bin> for DkcDummySink {}

impl DkcDummySink {
    fn new(bin: &Bin) -> Box<BinImpl<Bin>> {
        Box::new({
            Self {
                cat: gst::DebugCategory::new(
                    "dkcsink",
                    gst::DebugColorFlags::empty(),
                    "DankCaster dummy sink element",
                ),
            }
        })
    }

    fn class_init(klass: &mut BinClass) {
        klass.set_metadata(
            "DankCaster dummy sink element",
            "Audio/Video",
            "DankCaster dummy sink element",
            "Blaxar Waldarax <blaxar.waldarax@gmail.com>",
        );

        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let audio_caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        let video_sink_pad_template = gst::PadTemplate::new(
            "video_sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &video_caps,
        );

        let audio_sink_pad_template = gst::PadTemplate::new(
            "audio_sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &audio_caps,
        );

        klass.add_pad_template(video_sink_pad_template);
        klass.add_pad_template(audio_sink_pad_template);
    }
}


pub fn register(plugin: &gst::Plugin) {
    let type_ = register_type(DkcDummySinkStatic);
    gst::Element::register(plugin, "dkcdummysink", 0, type_);
}
