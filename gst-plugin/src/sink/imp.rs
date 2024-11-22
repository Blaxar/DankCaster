use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;

use std::sync::LazyLock;

#[derive(Default)]
pub struct DkcDummySink {

}

static CAT: LazyLock<gst::DebugCategory> = LazyLock::new(|| {
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
    fn constructed(&self) {
        let video_elem = gst::ElementFactory::make("autovideosink").name("testvideosink").build()
            .expect("Could not create video sink element.");
        let video_capsf = gst::ElementFactory::make("capsfilter").name("videocapsfilter").build()
            .expect("Could not create video capsfilter element.");
        let audio_elem = gst::ElementFactory::make("autoaudiosink").name("testaudiosink").build()
            .expect("Could not create audio sink element.");
        let audio_capsf = gst::ElementFactory::make("capsfilter").name("audiocapsfilter").build()
            .expect("Could not create audio capsfilter element.");

        self.add_element(&video_elem).expect("Could not add video element to this sink");
        self.add_element(&video_capsf).expect("Could not add video caps filter to this sink");
        self.add_element(&audio_elem).expect("Could not add audio element to this sink");
        self.add_element(&audio_capsf).expect("Could not add audio caps filter to this sink");

        video_capsf.link(&video_elem).expect("Could not link video capsfilter to audio element.");
        audio_capsf.link(&audio_elem).expect("Could not link audio capsfilter to audio element.");

        let video_caps = gst::Caps::new_empty_simple(
            "video/x-raw"
        );
        let audio_caps = gst::Caps::new_empty_simple(
            "audio/x-raw"
        );

        video_capsf.set_property("caps", &video_caps);
        audio_capsf.set_property("caps", &audio_caps);

        let video_pad = video_capsf.static_pad("sink").unwrap();
        let audio_pad = audio_capsf.static_pad("sink").unwrap();

        let video_ghost_pad = gst::GhostPad::builder(gst::PadDirection::Sink)
            .with_target(&video_pad).unwrap().name("video_sink").build();
        let audio_ghost_pad = gst::GhostPad::builder(gst::PadDirection::Sink)
            .with_target(&audio_pad).unwrap().name("audio_sink").build();

        let obj = self.obj();
        obj.add_pad(&video_ghost_pad).unwrap();
        obj.add_pad(&audio_ghost_pad).unwrap();
    }
}

impl GstObjectImpl for DkcDummySink {}

impl ElementImpl for DkcDummySink {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: LazyLock<gst::subclass::ElementMetadata> = LazyLock::new(|| {
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
        static PAD_TEMPLATES: LazyLock<Vec<gst::PadTemplate>> = LazyLock::new(|| {
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
