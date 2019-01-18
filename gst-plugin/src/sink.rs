use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use gst;
use gst::prelude::*;
use gst::subclass::prelude::*;

struct DkcDummySink {
    cat: gst::DebugCategory,
}

impl ObjectImpl for DkcDummySink {

    glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        let bin = obj.downcast_ref::<gst::Bin>().unwrap();

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

impl ElementImpl for DkcDummySink {}
impl BinImpl for DkcDummySink {}

impl ObjectSubclass for DkcDummySink {

    const NAME: &'static str = "DkcDummySink";
    type ParentType = gst::Bin;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {
        Self {
            cat: gst::DebugCategory::new(
                "dkcsink",
                gst::DebugColorFlags::empty(),
                "DankCaster dummy sink element",
            ),
        }
    }

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
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


pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(plugin, "dkcdummysink", 0, DkcDummySink::get_type())
}
