use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use gst;
use gst::prelude::*;
use gst::subclass::prelude::*;

struct DkcDummySource {
    cat: gst::DebugCategory,
}

impl ObjectImpl for DkcDummySource {

    glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        let bin = obj.downcast_ref::<gst::Bin>().unwrap();

        let video_elem = gst::ElementFactory::make("videotestsrc", Some("testvideosource"))
            .expect("Could not create video source element.");

        let audio_elem = gst::ElementFactory::make("audiotestsrc", Some("testaudiosource"))
            .expect("Could not create audio source element.");

        self.add_element(bin, &video_elem);
        self.add_element(bin, &audio_elem);

        let video_pad = video_elem.get_static_pad("src").unwrap();

        let audio_pad = audio_elem.get_static_pad("src").unwrap();

        let video_ghost_pad = gst::GhostPad::new(Some("video_src"), &video_pad).unwrap();

        let audio_ghost_pad = gst::GhostPad::new(Some("audio_src"), &audio_pad).unwrap();

        bin.add_pad(&video_ghost_pad).unwrap();
        bin.add_pad(&audio_ghost_pad).unwrap();
    }
}

impl ElementImpl for DkcDummySource {}
impl BinImpl for DkcDummySource {}

impl ObjectSubclass for DkcDummySource {

    const NAME: &'static str = "DkcDummySource";
    type ParentType = gst::Bin;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {
        Self {
            cat: gst::DebugCategory::new(
                "dkcsource",
                gst::DebugColorFlags::empty(),
                Some("DankCaster dummy source element"),
            )
        }
    }

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
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

        klass.add_pad_template(video_src_pad_template.unwrap());
        klass.add_pad_template(audio_src_pad_template.unwrap());
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(Some(plugin), "dkcdummysource", gst::Rank::None, DkcDummySource::get_type())
}
