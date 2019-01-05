use glib;
use gst;
use gst::prelude::*;
use gst_base::prelude::*;
use gst_plugin::bin::*;

use gst_plugin::object::*;
use gst_plugin::base_src::*;
use gst_plugin::element::*;
use gobject_subclass::object::*;

struct DkcDummySourceStatic;
struct DkcDummySource {
    cat: gst::DebugCategory,
}

impl ImplTypeStatic<Bin> for DkcDummySourceStatic {
    fn get_name(&self) -> &str {
        "DkcDummySource"
    }

    fn new(&self, element: &Bin) -> Box<BinImpl<Bin>> {
        DkcDummySource::new(element)
    }

    fn class_init(&self, klass: &mut BinClass) {
        DkcDummySource::class_init(klass);
    }
}

impl ObjectImpl<Bin> for DkcDummySource {
    fn constructed(&self, bin: &Bin) {
        bin.parent_constructed();

        let video_elem = gst::ElementFactory::make("videotestsrc", "testvideosource")
            .expect("Could not create video source element.");

        let audio_elem = gst::ElementFactory::make("audiotestsrc", "testaudiosource")
            .expect("Could not create audio source element.");

        self.add_element(bin, &video_elem);
        self.add_element(bin, &audio_elem);

        let video_pad = video_elem.get_static_pad("src").unwrap();

        let audio_pad = audio_elem.get_static_pad("src").unwrap();

        let video_ghost_pad = gst::GhostPad::new("video_src", &video_pad).unwrap();

        let audio_ghost_pad = gst::GhostPad::new("audio_src", &audio_pad).unwrap();

        bin.add_pad(&video_ghost_pad).unwrap();
        bin.add_pad(&audio_ghost_pad).unwrap();
    }
}

impl ElementImpl<Bin> for DkcDummySource {}
impl BinImpl<Bin> for DkcDummySource {}

impl DkcDummySource {
    fn new(bin: &Bin) -> Box<BinImpl<Bin>> {
        Box::new({
            Self {
                cat: gst::DebugCategory::new(
                    "dkcsource",
                    gst::DebugColorFlags::empty(),
                    "DankCaster dummy source element",
                ),
            }
        })
    }

    fn class_init(klass: &mut BinClass) {
        klass.set_metadata(
            "DankCaster dummy source element",
            "Audio/Video",
            "DankCaster dummy source element",
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

        let video_src_pad_template = gst::PadTemplate::new(
            "video_src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &video_caps,
        );

        let audio_src_pad_template = gst::PadTemplate::new(
            "audio_src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &audio_caps,
        );

        klass.add_pad_template(video_src_pad_template);
        klass.add_pad_template(audio_src_pad_template);
    }
}


pub fn register(plugin: &gst::Plugin) {
    let type_ = register_type(DkcDummySourceStatic);
    gst::Element::register(plugin, "dkcdummysource", 0, type_);
}
