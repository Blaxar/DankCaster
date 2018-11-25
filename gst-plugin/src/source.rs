use glib;
use gst;
use gst::prelude::*;
use gst_base::prelude::*;
use gst_plugin::bin::*;

use gst_plugin::object::*;
use gst_plugin::base_src::*;
use gst_plugin::element::*;
use gobject_subclass::object::*;

struct DkcSourceStatic;
struct DkcSource {
    cat: gst::DebugCategory,
}

impl ImplTypeStatic<Bin> for DkcSourceStatic {
    fn get_name(&self) -> &str {
        "DkcSource"
    }

    fn new(&self, element: &Bin) -> Box<BinImpl<Bin>> {
        DkcSource::new(element)
    }

    fn class_init(&self, klass: &mut BinClass) {
        DkcSource::class_init(klass);
    }
}

impl ObjectImpl<Bin> for DkcSource {
    fn constructed(&self, bin: &Bin) {
        bin.parent_constructed();

        let elem = gst::ElementFactory::make("videotestsrc", "testsource")
            .expect("Could not create source element.");
        let pad = elem.get_static_pad("src").unwrap();
        println!("{:?}", pad);

        self.add_element(bin, &elem);

        let ghost_pad = gst::GhostPad::new("src", &pad).unwrap(); //segfault
        println!("{:?}", ghost_pad);

        bin.add_pad(&ghost_pad).unwrap();
    }
}

impl ElementImpl<Bin> for DkcSource {}
impl BinImpl<Bin> for DkcSource {}

impl DkcSource {
    fn new(bin: &Bin) -> Box<BinImpl<Bin>> {
        Box::new({
            Self {
                cat: gst::DebugCategory::new(
                    "dkcsource",
                    gst::DebugColorFlags::empty(),
                    "DankCaster source element",
                ),
            }
        })
    }

    fn class_init(klass: &mut BinClass) {
        klass.set_metadata(
            "DankCaster source element",
            "Audio/Video",
            "DankCaster source element",
            "Blaxar Waldarax <blaxar.waldarax@gmail.com>",
        );

        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let src_pad_template = gst::PadTemplate::new(
            "src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &video_caps,
        );
        klass.add_pad_template(src_pad_template);
    }
}


pub fn register(plugin: &gst::Plugin) {
    let type_ = register_type(DkcSourceStatic);
    gst::Element::register(plugin, "dkcsource", 0, type_);
}
