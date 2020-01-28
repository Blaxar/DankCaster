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

        let video_elem = gst::ElementFactory::make("autovideosink", Some("testvideosink"))
            .expect("Could not create video sink element.");
        let video_capsf = gst::ElementFactory::make("capsfilter", Some("videocapsfilter"))
            .expect("Could not create video capsfilter element.");

        let audio_elem = gst::ElementFactory::make("autoaudiosink", Some("testaudiosink"))
            .expect("Could not create audio sink element.");
        let audio_capsf = gst::ElementFactory::make("capsfilter", Some("audiocapsfilter"))
            .expect("Could not create audio capsfilter element.");

        self.add_element(bin, &video_elem);
        self.add_element(bin, &video_capsf);
        self.add_element(bin, &audio_elem);
        self.add_element(bin, &audio_capsf);

        video_capsf.link(&video_elem).expect("Could not link video capsfilter to audio element.");
        audio_capsf.link(&audio_elem).expect("Could not link audio capsfilter to audio element.");

        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let audio_caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        video_capsf.set_property("caps", &video_caps).expect("Could not set caps on video capsfilter.");
        audio_capsf.set_property("caps", &audio_caps).expect("Could not set caps on audio capsfilter.");

        let video_pad = video_capsf.get_static_pad("sink").unwrap();
        let audio_pad = audio_capsf.get_static_pad("sink").unwrap();

        let video_ghost_pad = gst::GhostPad::new(Some("video_sink"), &video_pad).unwrap();
        let audio_ghost_pad = gst::GhostPad::new(Some("audio_sink"), &audio_pad).unwrap();

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
                Some("DankCaster dummy sink element"),
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

        klass.add_pad_template(video_sink_pad_template.unwrap());
        klass.add_pad_template(audio_sink_pad_template.unwrap());
    }
}


pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(Some(plugin), "dkcdummysink", gst::Rank::None, DkcDummySink::get_type())
}
