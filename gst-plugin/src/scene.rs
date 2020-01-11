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

    if tmpl_caps.is_strictly_equal(video_caps) {

        let mixer_pad = video_mixer.get_request_pad("sink_%u").unwrap();
        let ghost_pad_name = format!("video_{}", mixer_pad.get_name());

        /* Add ghost video sink pad to the element (targeting mixer sink) */
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &mixer_pad,
                                                         templ).unwrap();
        element.add_pad(&ghost_pad).expect("Could not add ghost pad to element");

        Some(ghost_pad.upcast::<gst::Pad>())

    } else if tmpl_caps.is_strictly_equal(audio_caps) {

        let mixer_pad = audio_mixer.get_request_pad("sink_%u").unwrap();
        let ghost_pad_name = format!("audio_{}", mixer_pad.get_name());

        /* Add ghost video sink pad to the element (targeting mixer sink) */
        let ghost_pad = gst::GhostPad::new_from_template(Some(&*ghost_pad_name),
                                                         &mixer_pad,
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
        match pad.get_direction() {
            gst::PadDirection::Sink => {
                let mixer_sink_pad = pad.downcast_ref::<gst::GhostPad>().unwrap().get_target().expect("Could not get mixer sink pad for releasing");
                let mixer = mixer_sink_pad.get_parent_element().expect("Could not get mixer element from its sink pad");

                /* release mixer sink pad */
                mixer.release_request_pad(&mixer_sink_pad);

                /* remove ghost pad */
                element.remove_pad(pad).expect("Could not remove ghost sink pad");
            },
            gst::PadDirection::Src => {
                let bin = element.downcast_ref::<gst::Bin>().unwrap();
                let queue_src_pad = pad.downcast_ref::<gst::GhostPad>().unwrap().get_target().expect("Could not get queue src pad");
                let queue = queue_src_pad.get_parent_element().expect("Could not get queue element from its src pad");
                let queue_sink_pad = queue.get_static_pad("sink").expect("Could not get queue sink pad");
                let tee_src_pad = queue_sink_pad.get_peer().expect("Could not get tee src pad its peer");
                let tee = tee_src_pad.get_parent_element().expect("Could not get tee element from its src pad");

                //release tee src pad
                tee.release_request_pad(&tee_src_pad);

                //remove ghost pad
                element.remove_pad(pad).expect("Could not remove ghost sink pad");

                //remove queue
                bin.remove(&queue);
            },
            _ => (),
        };
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
            video_mixer: gst::ElementFactory::make("videomixer", None)
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
            .expect("Could not get requet pad 0");
        let video_sink_1 = scene.get_request_pad("video_sink_%u")
            .expect("Could not get requet pad 1");

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
            .expect("Could not get requet pad 0");
        let audio_sink_1 = scene.get_request_pad("audio_sink_%u")
            .expect("Could not get requet pad 1");

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
            .expect("Could not get requet pad 0");
        let video_src_1 = scene.get_request_pad("video_src_%u")
            .expect("Could not get requet pad 1");

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
            .expect("Could not get requet pad 0");
        let audio_src_1 = scene.get_request_pad("audio_src_%u")
            .expect("Could not get requet pad 1");

        scene.release_request_pad(&audio_src_0);
        scene.release_request_pad(&audio_src_1);
    }

}
