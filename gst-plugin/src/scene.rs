use glib;
use gst;
use gst::prelude::*;
use gst_base::prelude::*;
use gst_plugin::bin::*;

use gst_plugin::object::*;
use gst_plugin::base_src::*;
use gst_plugin::element::*;
use gobject_subclass::object::*;

struct DkcSceneStatic;
struct DkcScene {
    cat: gst::DebugCategory,
    video_mixer: gst::Element,
    audio_mixer: gst::Element,
    video_tee: gst::Element,
    audio_tee: gst::Element,
}

impl ImplTypeStatic<Bin> for DkcSceneStatic {
    fn get_name(&self) -> &str {
        "DkcScene"
    }

    fn new(&self, element: &Bin) -> Box<BinImpl<Bin>> {
        DkcScene::new(element)
    }

    fn class_init(&self, klass: &mut BinClass) {
        DkcScene::class_init(klass);
    }
}

impl ObjectImpl<Bin> for DkcScene {
    fn constructed(&self, bin: &Bin) {
        bin.parent_constructed();

        self.add_element(bin, &self.video_mixer);
        self.add_element(bin, &self.audio_mixer);
        self.add_element(bin, &self.video_tee);
        self.add_element(bin, &self.audio_tee);

        self.video_mixer.link(&self.video_tee)
            .expect("Could not link video mixer to its tee element.");
        self.audio_mixer.link(&self.audio_tee)
            .expect("Could not link audio mixer to its tee element.");
    }
}

fn handle_sink_request(
    element: &Bin,
    templ: &gst::PadTemplate,
    tmpl_caps: &gst::Caps,
    video_mixer: &gst::Element,
    audio_mixer: &gst::Element,
    video_caps: &gst::Caps,
    audio_caps: &gst::Caps,
) -> Option<gst::Pad> {

    if tmpl_caps.is_strictly_equal(video_caps) {
        let mixer_pad = video_mixer.get_request_pad("sink_%u").unwrap();
        let ghost_pad_name = format!("video_{}", mixer_pad.get_name());
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &mixer_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");
        Some(ghost_pad.upcast::<gst::Pad>())
    } else if tmpl_caps.is_strictly_equal(audio_caps) {
        let mixer_pad = audio_mixer.get_request_pad("sink_%u").unwrap();
        let ghost_pad_name = format!("audio_{}", mixer_pad.get_name());
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &mixer_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");
        Some(ghost_pad.upcast::<gst::Pad>())
    } else {
        None
    }

}

fn handle_src_request(
    element: &Bin,
    templ: &gst::PadTemplate,
    tmpl_caps: &gst::Caps,
    video_tee: &gst::Element,
    audio_tee: &gst::Element,
    video_caps: &gst::Caps,
    audio_caps: &gst::Caps,
) -> Option<gst::Pad> {

    if tmpl_caps.is_strictly_equal(video_caps) {
        let tee_pad = video_tee.get_request_pad("src_%u").unwrap();
        let ghost_pad_name = format!("video_{}", tee_pad.get_name());
        let queue = gst::ElementFactory::make("queue", None)
            .expect("Could not create queue element for video mixer tee.");
        element.add(&queue).expect("Could not add queue element for video mixer to the bin");
        video_tee.link(&queue).expect("Could not link queue element to video tee");
        let queue_pad = queue.get_static_pad("src").unwrap();
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &queue_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");
        Some(ghost_pad.upcast::<gst::Pad>())
    } else if tmpl_caps.is_strictly_equal(audio_caps) {
        let tee_pad = audio_tee.get_request_pad("src_%u").unwrap();
        let ghost_pad_name = format!("audio_{}", tee_pad.get_name());
        let queue = gst::ElementFactory::make("queue", None)
            .expect("Could not create queue element for audio mixer tee.");
        element.add(&queue).expect("Could not add queue element for audio mixer to the bin");
        audio_tee.link(&queue).expect("Could not link queue element to audio tee");
        let queue_pad = queue.get_static_pad("src").unwrap();
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &queue_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");
        Some(ghost_pad.upcast::<gst::Pad>())
    } else {
        None
    }

}

impl ElementImpl<Bin> for DkcScene {
    fn request_new_pad(
        &self,
        element: &Bin,
        templ: &gst::PadTemplate,
        name: Option<String>,
        caps: Option<&gst::CapsRef>,
    ) -> Option<gst::Pad> {

        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let audio_caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        let tmpl_caps = templ.get_caps().unwrap();

        match templ.get_property_direction() {
            gst::PadDirection::Sink =>
                match caps {
                    Some(caps_ref) =>
                        if tmpl_caps.is_always_compatible(caps_ref) {
                            handle_sink_request(element, templ, &tmpl_caps,
                                                &self.video_mixer, &self.audio_mixer,
                                                &video_caps, &audio_caps)
                        } else {
                            None
                        },
                    None => handle_sink_request(element, templ, &tmpl_caps,
                                                &self.video_mixer, &self.audio_mixer,
                                                &video_caps, &audio_caps)
                },
            gst::PadDirection::Src =>
                match caps {
                    Some(caps_ref) =>
                        if tmpl_caps.is_always_compatible(caps_ref) {
                            handle_src_request(element, templ, &tmpl_caps,
                                               &self.video_tee, &self.audio_tee,
                                               &video_caps, &audio_caps)
                        } else {
                            None
                        },
                    None => handle_src_request(element, templ, &tmpl_caps,
                                               &self.video_tee, &self.audio_tee,
                                               &video_caps, &audio_caps)
                },
            _ => None,
        }
    }

}
impl BinImpl<Bin> for DkcScene {}

impl DkcScene {
    fn new(bin: &Bin) -> Box<BinImpl<Bin>> {
        Box::new({
            Self {
                cat: gst::DebugCategory::new(
                    "dkcsource",
                    gst::DebugColorFlags::empty(),
                    "DankCaster dummy source element",
                ),
                video_mixer: gst::ElementFactory::make("videomixer", None)
                    .expect("Could not create video source element."),
                audio_mixer: gst::ElementFactory::make("audiomixer", None)
                    .expect("Could not create audio source element."),
                video_tee: gst::ElementFactory::make("tee", None)
                    .expect("Could not create video tee element."),
                audio_tee: gst::ElementFactory::make("tee", None)
                    .expect("Could not create audio tee element."),
            }
        })
    }

    fn class_init(klass: &mut BinClass) {
        klass.set_metadata(
            "DankCaster scene element",
            "Audio/Video",
            "DankCaster scene element",
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
            "video_src_%u",
            gst::PadDirection::Src,
            gst::PadPresence::Request,
            &video_caps,
        );

        let audio_src_pad_template = gst::PadTemplate::new(
            "audio_src_%u",
            gst::PadDirection::Src,
            gst::PadPresence::Request,
            &audio_caps,
        );

        let video_sink_pad_template = gst::PadTemplate::new(
            "video_sink_%u",
            gst::PadDirection::Sink,
            gst::PadPresence::Request,
            &video_caps,
        );

        let audio_sink_pad_template = gst::PadTemplate::new(
            "audio_sink_%u",
            gst::PadDirection::Sink,
            gst::PadPresence::Request,
            &audio_caps,
        );

        klass.add_pad_template(video_sink_pad_template);
        klass.add_pad_template(audio_sink_pad_template);
        klass.add_pad_template(video_src_pad_template);
        klass.add_pad_template(audio_src_pad_template);
    }
}


pub fn register(plugin: &gst::Plugin) {
    let type_ = register_type(DkcSceneStatic);
    gst::Element::register(plugin, "dkcscene", 0, type_);
}
