use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;

use once_cell::sync::Lazy;

#[derive(Default)]
pub struct DkcDummySource {

}

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "dkcdummysource",
        gst::DebugColorFlags::empty(),
        Some("DankCaster dummy source element"),
    )
});

#[glib::object_subclass]
impl ObjectSubclass for DkcDummySource {
    const NAME: &'static str = "DkcDummySource";
    type Type = super::DkcDummySource;
    type ParentType = gst::Bin;
}

impl ObjectImpl for DkcDummySource {
    fn constructed(&self, obj: &Self::Type) {
        let video_elem = gst::ElementFactory::make("videotestsrc", Some("testvideosource"))
            .expect("Could not create video source element.");
        let audio_elem = gst::ElementFactory::make("audiotestsrc", Some("testaudiosource"))
            .expect("Could not create audio source element.");

        self.add_element(obj, &video_elem).expect("Could not add video element to this source");
        self.add_element(obj, &audio_elem).expect("Could not add audio element to this source");

        let video_pad = video_elem.static_pad("src").unwrap();
        let audio_pad = audio_elem.static_pad("src").unwrap();

        let video_ghost_pad = gst::GhostPad::with_target(Some("video_src"), &video_pad).unwrap();
        let audio_ghost_pad = gst::GhostPad::with_target(Some("audio_src"), &audio_pad).unwrap();

        obj.add_pad(&video_ghost_pad).unwrap();
        obj.add_pad(&audio_ghost_pad).unwrap();
    }
}

impl GstObjectImpl for DkcDummySource {}

impl ElementImpl for DkcDummySource {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "DankCaster Dummy Source",
                "Audio/Video",
                "DankCaster dummy source element",
                "Julien 'Blaxar' Bardagi <blaxar.waldarax@gmail.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            // src pad capabilities
            let video_caps = gst::Caps::builder("video/x-raw")
                .build();
            let audio_caps = gst::Caps::builder("audio/x-raw")
                .build();

            let video_src_pad_template = gst::PadTemplate::new(
                "video_src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &video_caps,
            )
            .unwrap();

            let audio_src_pad_template = gst::PadTemplate::new(
                "audio_src",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &audio_caps,
            )
            .unwrap();

            vec![video_src_pad_template, audio_src_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl BinImpl for DkcDummySource {}
