use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use gst;
use gst::prelude::*;
use gst::subclass::prelude::*;

struct DkcScene {
    cat: gst::DebugCategory,
    video_mixer: gst::Element,
    audio_mixer: gst::Element,
    video_tee: gst::Element,
    audio_tee: gst::Element,
}

fn try_set_property<T: glib::StaticVariantType + glib::variant::FromVariant + glib::value::SetValue>
    (mixer_pad: &gst::Pad,
     param_name: &str,
     param_value: &glib::variant::Variant) -> bool {
    if param_value.is::<T>(){
        match mixer_pad.set_property(param_name, &param_value.get::<T>().unwrap()) {
            Ok(_) => true,
            Err(_) => false,
        }
    } else { false }
}

impl DkcScene {
    fn class_update_video_input_handler(pad: &gst::Pad, param_name: &str, param_value: &glib::variant::Variant)
                                        -> Option<glib::value::Value> {

        let mixer_pad = pad.downcast_ref::<gst::GhostPad>().expect("Could not cast Pad as GhostPad")
            .get_target().expect("Could not get ghost pad target pad (queue sink pad)")
            .get_parent_element().expect("Could not get queue")
            .get_static_pad("src").expect("Could not get queue src")
            .get_peer().expect("Could not get queue src peer");

        match param_name {
            "width" | "height" | "xpos" | "ypos" =>
                Some(try_set_property::<i32>(&mixer_pad, param_name, param_value).to_value()),
            "alpha" => Some(try_set_property::<f64>(&mixer_pad, param_name, param_value).to_value()),
            "zorder" => Some(try_set_property::<u32>(&mixer_pad, param_name, param_value).to_value()),
            _ => Some(false.to_value())
        }
    }

    fn class_update_audio_input_handler(pad: &gst::Pad, param_name: &str, param_value: &glib::variant::Variant)
                                        -> Option<glib::value::Value> {

        let mixer_pad = pad.downcast_ref::<gst::GhostPad>().expect("Could not cast Pad as GhostPad")
            .get_target().expect("Could not get ghost pad target pad (queue sink pad)")
            .get_parent_element().expect("Could not get queue")
            .get_static_pad("src").expect("Could not get queue src")
            .get_peer().expect("Could not get queue src peer");

        match param_name {
            "mute" =>
                Some(try_set_property::<bool>(&mixer_pad, param_name, param_value).to_value()),
            "volume" => Some(try_set_property::<f64>(&mixer_pad, param_name, param_value).to_value()),
            _ => Some(false.to_value())
        }
    }

    fn class_update_input_handler(scht_: &glib::subclass::types::SignalClassHandlerToken, v: &[glib::value::Value])
                                  -> Option<glib::value::Value> {

        let bin = v[0].get::<gst::Bin>().unwrap().unwrap(); // It's a Result containing an Option
        let sink_pad_name = v[1].get::<String>().unwrap();
        let param_name = v[2].get::<String>().unwrap();
        let param_value = v[3].get::<glib::variant::Variant>().unwrap();

        if sink_pad_name.is_none() || param_name.is_none() || param_value.is_none() {
            return Some(false.to_value());
        }

        let sink_pad_name = sink_pad_name.unwrap();
        let param_name = param_name.unwrap();
        let param_value = param_value.unwrap();

        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let audio_caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        match bin.get_static_pad(&sink_pad_name) {
            Some(pad) => match pad.get_direction() {
                gst::PadDirection::Sink => {
                    match pad.get_pad_template_caps() {
                        Some(caps_ref) =>
                            if video_caps.is_strictly_equal(&caps_ref) {
                                Self::class_update_video_input_handler(&pad, &param_name, &param_value)
                            } else if audio_caps.is_strictly_equal(&caps_ref) {
                                Self::class_update_audio_input_handler(&pad, &param_name, &param_value)
                            } else {
                                Some(false.to_value()) // Pad is neither of the video nor the audio type
                            },
                        None => { Some(false.to_value()) } // No template found on this pad
                    }
                },
                _ => Some(false.to_value()) // Not a Sink, therefore not an input
            },
            None => Some(false.to_value()) // Pad not found (with this name)
        }
    }
}

impl ObjectImpl for DkcScene {

    glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        let bin = obj.downcast_ref::<gst::Bin>().unwrap();

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
    element: &gst::Bin,
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
    let queue_sink_pad = queue.get_static_pad("sink").unwrap();
    let queue_src_pad = queue.get_static_pad("src").unwrap();

    if tmpl_caps.is_strictly_equal(video_caps) {

        let mixer_pad = video_mixer.get_request_pad("sink_%u").unwrap();
        let ghost_pad_name = format!("video_{}", mixer_pad.get_name());
        queue_src_pad.link(&mixer_pad).expect("Could not link queue element to video mixer");

        /* Add ghost video sink pad to the element (targeting mixer sink) */
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &queue_sink_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else if tmpl_caps.is_strictly_equal(audio_caps) {

        let mixer_pad = audio_mixer.get_request_pad("sink_%u").unwrap();
        let ghost_pad_name = format!("audio_{}", mixer_pad.get_name());
        queue_src_pad.link(&mixer_pad).expect("Could not link queue element to audio mixer");

        /* Add ghost video sink pad to the element (targeting mixer sink) */
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &queue_sink_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else { None }

}

fn handle_src_request(
    element: &gst::Bin,
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

        /* Create, add and link tee queue */
        let queue = gst::ElementFactory::make("queue", None)
            .expect("Could not create queue element for video mixer tee.");
        element.add(&queue).expect("Could not add queue element for video mixer to the bin");
        video_tee.link(&queue).expect("Could not link queue element to video tee");

        /* Add ghost video src pad to the element (targeting queue src) */
        let queue_pad = queue.get_static_pad("src").unwrap();
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &queue_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else if tmpl_caps.is_strictly_equal(audio_caps) {

        let tee_pad = audio_tee.get_request_pad("src_%u").unwrap();
        let ghost_pad_name = format!("audio_{}", tee_pad.get_name());

        /* Create, add and link tee queue */
        let queue = gst::ElementFactory::make("queue", None)
            .expect("Could not create queue element for audio mixer tee.");
        element.add(&queue).expect("Could not add queue element for audio mixer to the bin");
        audio_tee.link(&queue).expect("Could not link queue element to audio tee");

        /* Add ghost audio src pad to the element (targeting queue src) */
        let queue_pad = queue.get_static_pad("src").unwrap();
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &queue_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else { None }

}

impl ElementImpl for DkcScene {
    fn request_new_pad(
        &self,
        element: &gst::Element,
        templ: &gst::PadTemplate,
        name: Option<String>,
        caps: Option<&gst::Caps>,
    ) -> Option<gst::Pad> {

        let bin = element.downcast_ref::<gst::Bin>().unwrap();

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
                            handle_sink_request(bin, templ, &tmpl_caps,
                                                &self.video_mixer, &self.audio_mixer,
                                                &video_caps, &audio_caps)
                        } else {
                            None
                        },
                    None => handle_sink_request(bin, templ, &tmpl_caps,
                                                &self.video_mixer, &self.audio_mixer,
                                                &video_caps, &audio_caps)
                },
            gst::PadDirection::Src =>
                match caps {
                    Some(caps_ref) =>
                        if tmpl_caps.is_always_compatible(caps_ref) {
                            handle_src_request(bin, templ, &tmpl_caps,
                                               &self.video_tee, &self.audio_tee,
                                               &video_caps, &audio_caps)
                        } else {
                            None
                        },
                    None => handle_src_request(bin, templ, &tmpl_caps,
                                               &self.video_tee, &self.audio_tee,
                                               &video_caps, &audio_caps)
                },
            _ => None,
        }
    }

    fn release_pad(&self, element: &gst::Element, pad: &gst::Pad) {
        let bin = element.downcast_ref::<gst::Bin>().unwrap();

        match pad.get_direction() {
            gst::PadDirection::Sink => {
                match pad.downcast_ref::<gst::GhostPad>().unwrap().get_target() {
                    Some(queue_sink_pad) => {
                        let queue = queue_sink_pad.get_parent_element().expect("Could not get queue element from its sink pad");
                        let mixer_sink_pad = queue.get_static_pad("src").expect("Could not get queue src")
                            .get_peer().expect("Could not get queue src peer");
                        let mixer = mixer_sink_pad.get_parent_element().expect("Could not get mixer");

                        // release mixer sink pad
                        mixer.release_request_pad(&mixer_sink_pad);

                        // remove queue
                        bin.remove(&queue);
                    },
                    None => ()
                }
            },
            gst::PadDirection::Src => {
                match pad.downcast_ref::<gst::GhostPad>().unwrap().get_target() {
                    Some(queue_src_pad) => {
                        let queue = queue_src_pad.get_parent_element().expect("Could not get queue element from its src pad");
                        let tee_src_pad = queue.get_static_pad("sink").expect("Could not get queue sink pad")
                            .get_peer().expect("Could not get tee src pad its peer");
                        let tee = tee_src_pad.get_parent_element().expect("Could not get tee element from its src pad");

                        // release tee src pad
                        tee.release_request_pad(&tee_src_pad);

                        // remove queue
                        bin.remove(&queue);
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

impl ObjectSubclass for DkcScene {

    const NAME: &'static str = "DkcDummyScene";
    type ParentType = gst::Bin;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {
        Self {
            cat: gst::DebugCategory::new(
                "dkcscene",
                gst::DebugColorFlags::empty(),
                Some("DankCaster dummy scene element"),
            ),
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

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
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

        klass.add_pad_template(video_sink_pad_template.unwrap());
        klass.add_pad_template(audio_sink_pad_template.unwrap());
        klass.add_pad_template(video_src_pad_template.unwrap());
        klass.add_pad_template(audio_src_pad_template.unwrap());

        klass.add_signal_with_class_handler("update-input", glib::SignalFlags::ACTION | glib::SignalFlags::RUN_LAST,
                                            &[glib::types::Type::String,
                                              glib::types::Type::String,
                                              glib::types::Type::Variant],
                                            glib::types::Type::Bool,
                                            DkcScene::class_update_input_handler);
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(Some(plugin), "dkcscene", gst::Rank::None, DkcScene::get_type())
}

#[cfg(test)]
mod tests {

    use super::*;
    use plugin_desc::plugin_register_static;

    fn set_up() {
        use std::sync::{Once, ONCE_INIT};
        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| {
            gst::init().unwrap();
            plugin_register_static();
        });
    }

    #[test]
    fn test_new() {
        set_up();

        assert!(
            match gst::ElementFactory::make("dkcscene", Some("scene")) {
                Ok(_) => true,
                Err(_) => false
            }
        );
    }

    #[test]
    fn test_video_sink_pad_request() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let video_sink_0 = scene.get_request_pad("video_sink_%u");
        let video_sink_1 = scene.get_request_pad("video_sink_%u");

        assert!(match video_sink_0 {
            Some(pad) => pad.get_name() == "video_sink_0",
            None => false
        });
        assert!(match video_sink_1 {
            Some(pad) => pad.get_name() == "video_sink_1",
            None => false
        });
    }

    #[test]
    fn test_video_sink_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let video_sink_0 = scene.get_request_pad("video_sink_%u")
            .expect("Could not get request pad 0");
        let video_sink_1 = scene.get_request_pad("video_sink_%u")
            .expect("Could not get request pad 1");

        scene.release_request_pad(&video_sink_0);
        scene.release_request_pad(&video_sink_1);
    }

    #[test]
    fn test_audio_sink_pad_request() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_sink_0 = scene.get_request_pad("audio_sink_%u");
        let audio_sink_1 = scene.get_request_pad("audio_sink_%u");

        assert!(match audio_sink_0 {
            Some(pad) => pad.get_name() == "audio_sink_0",
            None => false
        });
        assert!(match audio_sink_1 {
            Some(pad) => pad.get_name() == "audio_sink_1",
            None => false
        });
    }

    #[test]
    fn test_audio_sink_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_sink_0 = scene.get_request_pad("audio_sink_%u")
            .expect("Could not get request pad 0");
        let audio_sink_1 = scene.get_request_pad("audio_sink_%u")
            .expect("Could not get request pad 1");

        scene.release_request_pad(&audio_sink_0);
        scene.release_request_pad(&audio_sink_1);
    }

    #[test]
    fn test_video_src_pad_request() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let video_src_0 = scene.get_request_pad("video_src_%u");
        let video_src_1 = scene.get_request_pad("video_src_%u");

        assert!(match video_src_0 {
            Some(pad) => pad.get_name() == "video_src_0",
            None => false
        });
        assert!(match video_src_1 {
            Some(pad) => pad.get_name() == "video_src_1",
            None => false
        });
    }

    #[test]
    fn test_video_src_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let video_src_0 = scene.get_request_pad("video_src_%u")
            .expect("Could not get request pad 0");
        let video_src_1 = scene.get_request_pad("video_src_%u")
            .expect("Could not get request pad 1");

        scene.release_request_pad(&video_src_0);
        scene.release_request_pad(&video_src_1);
    }

    #[test]
    fn test_audio_src_pad_request() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_src_0 = scene.get_request_pad("audio_src_%u");
        let audio_src_1 = scene.get_request_pad("audio_src_%u");

        assert!(match audio_src_0 {
            Some(pad) => pad.get_name() == "audio_src_0",
            None => false
        });
        assert!(match audio_src_1 {
            Some(pad) => pad.get_name() == "audio_src_1",
            None => false
        });
    }

    #[test]
    fn test_audio_src_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_src_0 = scene.get_request_pad("audio_src_%u")
            .expect("Could not get request pad 0");
        let audio_src_1 = scene.get_request_pad("audio_src_%u")
            .expect("Could not get request pad 1");

        scene.release_request_pad(&audio_src_0);
        scene.release_request_pad(&audio_src_1);
    }

    #[test]
    fn test_update_input_action() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");

        /* Testing video pad */

        let video_sink_0 = scene.get_request_pad("video_sink_%u")
            .expect("Could not get request pad 0");

        // Those parameters have valid value types.
        assert!(scene.emit("update-input", &[&"video_sink_0", &"width", &(300 as i32).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(scene.emit("update-input", &[&"video_sink_0", &"height", &(300 as i32).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(scene.emit("update-input", &[&"video_sink_0", &"xpos", &(300 as i32).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(scene.emit("update-input", &[&"video_sink_0", &"ypos", &(300 as i32).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(scene.emit("update-input", &[&"video_sink_0", &"alpha", &(0.5 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(scene.emit("update-input", &[&"video_sink_0", &"zorder", &(3 as u32).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());

        // Those parameters have invalid value types.
        assert!(!scene.emit("update-input", &[&"video_sink_0", &"width", &(3.0 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(!scene.emit("update-input", &[&"video_sink_0", &"height", &(3.0 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(!scene.emit("update-input", &[&"video_sink_0", &"xpos", &(3.0 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(!scene.emit("update-input", &[&"video_sink_0", &"ypos", &(3.0 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(!scene.emit("update-input", &[&"video_sink_0", &"alpha", &(300 as i32).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(!scene.emit("update-input", &[&"video_sink_0", &"zorder", &(3.0 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());

        // This parameter does not exist.
        assert!(!scene.emit("update-input", &[&"video_sink_0", &"what", &(3.0 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());

        /* Testing audio pad */

        let audio_sink_0 = scene.get_request_pad("audio_sink_%u")
            .expect("Could not get request pad 0");

        // Those parameters have valid value types.
        assert!(scene.emit("update-input", &[&"audio_sink_0", &"mute", &true.to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(scene.emit("update-input", &[&"audio_sink_0", &"mute", &false.to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(scene.emit("update-input", &[&"audio_sink_0", &"volume", &(0.5 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());

        // Those parameters have invalid value types.
        assert!(!scene.emit("update-input", &[&"audio_sink_0", &"emit-signals", &(1).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(!scene.emit("update-input", &[&"audio_sink_0", &"mute", &(1).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
        assert!(!scene.emit("update-input", &[&"audio_sink_0", &"volume", &(1).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());

        // This parameter does not exist.
        assert!(!scene.emit("update-input", &[&"audio_sink_0", &"what", &(3.0 as f64).to_variant()])
                .unwrap().unwrap().get::<bool>().unwrap().unwrap());
    }

}
