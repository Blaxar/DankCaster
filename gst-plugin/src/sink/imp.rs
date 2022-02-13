use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;

use once_cell::sync::Lazy;

#[derive(Default)]
pub struct DkcDummySink {

}

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "dkcdummysink",
        gst::DebugColorFlags::empty(),
        Some("DankCaster dummy sink element"),
    )
});

#[glib::object_subclass]
impl ObjectSubclass for DkcDummySink {
    const NAME: &'static str = "DkcDummySink";
    type Type = super::DkcDummySink;
    type ParentType = gst::Bin;
}

impl ObjectImpl for DkcDummySink {
    fn constructed(&self, obj: &Self::Type) {
        let video_elem = gst::ElementFactory::make("autovideosink", Some("testvideosink"))
            .expect("Could not create video sink element.");
        let video_capsf = gst::ElementFactory::make("capsfilter", Some("videocapsfilter"))
            .expect("Could not create video capsfilter element.");
        let audio_elem = gst::ElementFactory::make("autoaudiosink", Some("testaudiosink"))
            .expect("Could not create audio sink element.");
        let audio_capsf = gst::ElementFactory::make("capsfilter", Some("audiocapsfilter"))
            .expect("Could not create audio capsfilter element.");

        self.add_element(obj, &video_elem).expect("Could not add video element to this sink");
        self.add_element(obj, &video_capsf).expect("Could not add video caps filter to this sink");
        self.add_element(obj, &audio_elem).expect("Could not add audio element to this sink");
        self.add_element(obj, &audio_capsf).expect("Could not add audio caps filter to this sink");

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

        video_capsf.set_property("caps", &video_caps);
        audio_capsf.set_property("caps", &audio_caps);

        let video_pad = video_capsf.static_pad("sink").unwrap();
        let audio_pad = audio_capsf.static_pad("sink").unwrap();

        let video_ghost_pad = gst::GhostPad::with_target(Some("video_sink"), &video_pad).unwrap();
        let audio_ghost_pad = gst::GhostPad::with_target(Some("audio_sink"), &audio_pad).unwrap();

        obj.add_pad(&video_ghost_pad).unwrap();
        obj.add_pad(&audio_ghost_pad).unwrap();
    }
}

impl GstObjectImpl for DkcDummySink {}

impl ElementImpl for DkcDummySink {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "DankCaster Dummy Sink",
                "Audio/Video",
                "DankCaster dummy sink element",
                "Julien 'Blaxar' Bardagi <blaxar.waldarax@gmail.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            // sink pad capabilities
            let video_caps = gst::Caps::builder("video/x-raw")
                .build();
            let audio_caps = gst::Caps::builder("audio/x-raw")
                .build();

            let video_sink_pad_template = gst::PadTemplate::new(
                "video_sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &video_caps,
            )
            .unwrap();

            let audio_sink_pad_template = gst::PadTemplate::new(
                "audio_sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &audio_caps,
            )
            .unwrap();

            vec![video_sink_pad_template, audio_sink_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl BinImpl for DkcDummySink {}
