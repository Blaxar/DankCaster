use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;
use std::sync::Mutex;

use once_cell::sync::Lazy;

struct State {
    video_mixer: gst::Element,
    audio_mixer: gst::Element,
    video_tee: gst::Element,
    audio_tee: gst::Element
}

impl State {
    fn new() -> Self {
        Self {
            video_mixer: gst::ElementFactory::make("compositor", None)
                .expect("Could not create video source element."),
            audio_mixer: gst::ElementFactory::make("audiomixer", None)
                .expect("Could not create audio source element."),
            video_tee: gst::ElementFactory::make("tee", None)
                .expect("Could not create video tee element."),
            audio_tee: gst::ElementFactory::make("tee", None)
                .expect("Could not create audio tee element."),
        }
    }
}

#[derive(Default)]
pub struct DkcScene {
    state: Mutex<Option<State>>
}

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "dkcscene",
        gst::DebugColorFlags::empty(),
        Some("DankCaster scene element"),
    )
});

impl DkcScene {
    fn class_update_video_input_handler(pad: &gst::Pad, param_name: &str, param_value: &glib::variant::Variant)
                                        -> Option<glib::value::Value> {

        let mixer_pad = pad.downcast_ref::<gst::GhostPad>().expect("Could not cast Pad as GhostPad")
            .target().expect("Could not get ghost pad target pad (queue sink pad)")
            .parent_element().expect("Could not get queue")
            .static_pad("src").expect("Could not get queue src")
            .peer().expect("Could not get queue src peer");

        match param_name {
            "width" | "height" | "xpos" | "ypos" => match param_value.get::<i32>() {
                Some(value) => { mixer_pad.set_property(param_name, value); Some(true.to_value()) },
                _ => Some(false.to_value())
            },
            "alpha" => match param_value.get::<f64>() {
                Some(value) => { mixer_pad.set_property(param_name, value); Some(true.to_value()) },
                _ => Some(false.to_value())
            },
            "zorder" => match param_value.get::<u32>() {
                Some(value) => { mixer_pad.set_property(param_name, value); Some(true.to_value()) },
                _ => Some(false.to_value())
            },
            _ => Some(false.to_value())
        }
    }

    fn class_update_audio_input_handler(pad: &gst::Pad, param_name: &str, param_value: &glib::variant::Variant)
                                        -> Option<glib::value::Value> {

        let mixer_pad = pad.downcast_ref::<gst::GhostPad>().expect("Could not cast Pad as GhostPad")
            .target().expect("Could not get ghost pad target pad (queue sink pad)")
            .parent_element().expect("Could not get queue")
            .static_pad("src").expect("Could not get queue src")
            .peer().expect("Could not get queue src peer");

        match param_name {
            "mute" => match param_value.get::<bool>() {
                Some(value) => { mixer_pad.set_property(param_name, value); Some(true.to_value()) },
                _ => Some(false.to_value())
            },
            "volume" => match param_value.get::<f64>() {
                Some(value) => { mixer_pad.set_property(param_name, value); Some(true.to_value()) },
                _ => Some(false.to_value())
            },
            _ => Some(false.to_value())
        }
    }

    fn class_update_input_handler(_token: &glib::subclass::signal::SignalClassHandlerToken, args: &[glib::value::Value])
                                  -> Option<glib::value::Value> {

        let bin = args[0].get::<super::DkcScene>().expect("signal arg");
        let sink_pad_name = args[1].get::<String>().expect("signal arg");
        let param_name = args[2].get::<String>().expect("signal arg");
        let param_value = args[3].get::<glib::variant::Variant>().expect("signal arg");

        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let audio_caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        match bin.static_pad(&sink_pad_name) {
            Some(pad) => match pad.direction() {
                gst::PadDirection::Sink => {
                    if video_caps.is_strictly_equal(pad.pad_template_caps().as_ref()) {
                        Self::class_update_video_input_handler(&pad, &param_name, &param_value)
                    } else if audio_caps.is_strictly_equal(pad.pad_template_caps().as_ref()) {
                        Self::class_update_audio_input_handler(&pad, &param_name, &param_value)
                    } else {
                        Some(false.to_value()) // Pad is neither of the video nor the audio type
                    }
                },
                _ => Some(false.to_value()) // Not a Sink, therefore not an input
            },
            None => Some(false.to_value()) // Pad not found (with this name)
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for DkcScene {
    const NAME: &'static str = "DkcScene";
    type Type = super::DkcScene;
    type ParentType = gst::Bin;
}

impl ObjectImpl for DkcScene {
    fn constructed(&self, obj: &Self::Type) {
        let state = State::new();

        self.add_element(obj, &state.video_mixer).expect("Could not add video mixer to bin.");
        self.add_element(obj, &state.audio_mixer).expect("Could not add audio mixer to bin");
        self.add_element(obj, &state.video_tee).expect("Could not add video tee to bin");
        self.add_element(obj, &state.audio_tee).expect("Could not add audio tee to bin");

        state.video_mixer.link(&state.video_tee)
            .expect("Could not link video mixer to its tee element.");
        state.audio_mixer.link(&state.audio_tee)
            .expect("Could not link audio mixer to its tee element.");

        *self.state.lock().unwrap() = Some(state);
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<glib::subclass::Signal>> = Lazy::new(|| {
            vec![
                glib::subclass::Signal::builder(
                    "update-input",
                    &[String::static_type().into(), String::static_type().into(),
                      glib::variant::Variant::static_type().into()],
                    bool::static_type().into(),
                )
                .action()
                .class_handler(&DkcScene::class_update_input_handler)
                .build()
            ]
        });

        SIGNALS.as_ref()
    }
}

impl GstObjectImpl for DkcScene {}

fn handle_sink_request(
    element: &crate::scene::DkcScene,
    templ: &gst::PadTemplate,
    tmpl_caps: &gst::Caps,
    video_mixer: &gst::Element,
    audio_mixer: &gst::Element,
    video_caps: &gst::Caps,
    audio_caps: &gst::Caps,
) -> Option<gst::Pad> {

    let queue = gst::ElementFactory::make("queue", None)
            .expect("Could not create input queue element.");
    element.add(&queue).expect("Could not add input queue element to the bin");
    let queue_sink_pad = queue.static_pad("sink").unwrap();
    let queue_src_pad = queue.static_pad("src").unwrap();

    if tmpl_caps.is_strictly_equal(video_caps) {

        let mixer_pad = video_mixer.request_pad_simple("sink_%u").unwrap();
        let ghost_pad_name = format!("video_{}", mixer_pad.name());
        queue_src_pad.link(&mixer_pad).expect("Could not link queue element to video mixer");

        /* Add ghost video sink pad to the element (targeting mixer sink) */
        let ghost_pad = gst::GhostPad::from_template_with_target(&templ,
                                                                 Some(&*ghost_pad_name),
                                                                 &queue_sink_pad).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else if tmpl_caps.is_strictly_equal(audio_caps) {

        let mixer_pad = audio_mixer.request_pad_simple("sink_%u").unwrap();
        let ghost_pad_name = format!("audio_{}", mixer_pad.name());
        queue_src_pad.link(&mixer_pad).expect("Could not link queue element to audio mixer");

        /* Add ghost video sink pad to the element (targeting mixer sink) */
        let ghost_pad = gst::GhostPad::from_template_with_target(templ,
                                                                 Some(&*ghost_pad_name),
                                                                 &queue_sink_pad).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else { None }

}

fn handle_src_request(
    element: &crate::scene::DkcScene,
    templ: &gst::PadTemplate,
    tmpl_caps: &gst::Caps,
    video_tee: &gst::Element,
    audio_tee: &gst::Element,
    video_caps: &gst::Caps,
    audio_caps: &gst::Caps,
) -> Option<gst::Pad> {

    if tmpl_caps.is_strictly_equal(video_caps) {

        let tee_pad = video_tee.request_pad_simple("src_%u").unwrap();
        let ghost_pad_name = format!("video_{}", tee_pad.name());

        /* Create, add and link tee queue */
        let queue = gst::ElementFactory::make("queue", None)
            .expect("Could not create queue element for video mixer tee.");
        element.add(&queue).expect("Could not add queue element for video mixer to the bin");
        video_tee.link(&queue).expect("Could not link queue element to video tee");

        /* Add ghost video src pad to the element (targeting queue src) */
        let queue_pad = queue.static_pad("src").unwrap();
        let ghost_pad = gst::GhostPad::from_template_with_target(templ,
                                                                 Some(&*ghost_pad_name),
                                                                 &queue_pad).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else if tmpl_caps.is_strictly_equal(audio_caps) {

        let tee_pad = audio_tee.request_pad_simple("src_%u").unwrap();
        let ghost_pad_name = format!("audio_{}", tee_pad.name());

        /* Create, add and link tee queue */
        let queue = gst::ElementFactory::make("queue", None)
            .expect("Could not create queue element for audio mixer tee.");
        element.add(&queue).expect("Could not add queue element for audio mixer to the bin");
        audio_tee.link(&queue).expect("Could not link queue element to audio tee");

        /* Add ghost audio src pad to the element (targeting queue src) */
        let queue_pad = queue.static_pad("src").unwrap();
        let ghost_pad = gst::GhostPad::from_template_with_target(templ,
                                                                 Some(&*ghost_pad_name),
                                                                 &queue_pad).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else { None }

}

impl ElementImpl for DkcScene {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "DankCaster Scene",
                "Audio/Video",
                "DankCaster scene element",
                "Julien 'Blaxar' Bardagi <blaxar.waldarax@gmail.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
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
            ).expect("");

            let audio_src_pad_template = gst::PadTemplate::new(
                "audio_src_%u",
                gst::PadDirection::Src,
                gst::PadPresence::Request,
                &audio_caps,
            ).expect("");

            let video_sink_pad_template = gst::PadTemplate::new(
                "video_sink_%u",
                gst::PadDirection::Sink,
                gst::PadPresence::Request,
                &video_caps,
            ).expect("");

            let audio_sink_pad_template = gst::PadTemplate::new(
                "audio_sink_%u",
                gst::PadDirection::Sink,
                gst::PadPresence::Request,
                &audio_caps,
            ).expect("");

            vec![video_src_pad_template, audio_src_pad_template, video_sink_pad_template, audio_sink_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }

    fn request_new_pad(
        &self,
        element: &Self::Type,
        templ: &gst::PadTemplate,
        _name: Option<String>,
        caps: Option<&gst::Caps>,
    ) -> Option<gst::Pad> {
        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let audio_caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        let state_lock = self.state.lock().unwrap();
        let state = state_lock.as_ref().unwrap();

        let tmpl_caps = templ.caps();

        match templ.direction() {
            gst::PadDirection::Sink =>
                match caps {
                    Some(caps_ref) =>
                        if tmpl_caps.is_always_compatible(caps_ref) {
                            handle_sink_request(element, templ, &tmpl_caps,
                                                &state.video_mixer, &state.audio_mixer,
                                                &video_caps, &audio_caps)
                        } else {
                            None
                        },
                    None => handle_sink_request(element, templ, &tmpl_caps,
                                                &state.video_mixer, &state.audio_mixer,
                                                &video_caps, &audio_caps)
                },
            gst::PadDirection::Src =>
                match caps {
                    Some(caps_ref) =>
                        if tmpl_caps.is_always_compatible(caps_ref) {
                            handle_src_request(element, templ, &tmpl_caps,
                                               &state.video_tee, &state.audio_tee,
                                               &video_caps, &audio_caps)
                        } else {
                            None
                        },
                    None => handle_src_request(element, templ, &tmpl_caps,
                                               &state.video_tee, &state.audio_tee,
                                               &video_caps, &audio_caps)
                },
            _ => None,
        }
    }

    fn release_pad(&self, element: &Self::Type, pad: &gst::Pad) {
        match pad.direction() {
            gst::PadDirection::Sink => {
                match pad.downcast_ref::<gst::GhostPad>().unwrap().target() {
                    Some(queue_sink_pad) => {
                        let queue = queue_sink_pad.parent_element().expect("Could not get queue element from its sink pad");
                        let mixer_sink_pad = queue.static_pad("src").expect("Could not get queue src")
                            .peer().expect("Could not get queue src peer");
                        let mixer = mixer_sink_pad.parent_element().expect("Could not get mixer");

                        // release mixer sink pad
                        mixer.release_request_pad(&mixer_sink_pad);

                        // remove queue
                        element.remove(&queue).expect("Could not remove sink queue from bin");
                    },
                    None => ()
                }
            },
            gst::PadDirection::Src => {
                match pad.downcast_ref::<gst::GhostPad>().unwrap().target() {
                    Some(queue_src_pad) => {
                        let queue = queue_src_pad.parent_element().expect("Could not get queue element from its src pad");
                        let tee_src_pad = queue.static_pad("sink").expect("Could not get queue sink pad")
                            .peer().expect("Could not get tee src pad its peer");
                        let tee = tee_src_pad.parent_element().expect("Could not get tee element from its src pad");

                        // release tee src pad
                        tee.release_request_pad(&tee_src_pad);

                        // remove queue
                        element.remove(&queue).expect("Could not remove src queue from bin");
                    },
                    None => ()
                }
            },
            _ => (),
        };

        /* remove ghost pad */
        element.remove_pad(pad).expect("Could not remove ghost sink pad");
    }
}

impl BinImpl for DkcScene {}
