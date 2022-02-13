use gst::glib;
use gst::prelude::*;

mod imp;

glib::wrapper! {
    pub struct DkcScene(ObjectSubclass<imp::DkcScene>) @extends gst::Bin, gst::Element, gst::Object;
}

// GStreamer elements need to be thread-safe. For the private implementation this is automatically
// enforced but for the public wrapper type we need to specify this manually.
unsafe impl Send for DkcScene {}
unsafe impl Sync for DkcScene {}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "dkcscene",
        gst::Rank::None,
        DkcScene::static_type(),
    )
}

#[cfg(test)]
mod tests {

    use super::super::*;
    use gst::prelude::*;

    fn set_up() {
        use std::sync::{Once};
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            gst::init().unwrap();
            plugin_desc::plugin_register_static().expect("Could not register Dkc plugin");
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
        let video_sink_0 = scene.request_pad_simple("video_sink_%u");
        let video_sink_1 = scene.request_pad_simple("video_sink_%u");

        assert!(match video_sink_0 {
            Some(pad) => pad.name() == "video_sink_0",
            None => false
        });
        assert!(match video_sink_1 {
            Some(pad) => pad.name() == "video_sink_1",
            None => false
        });
    }

    #[test]
    fn test_video_sink_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let video_sink_0 = scene.request_pad_simple("video_sink_%u")
            .expect("Could not get request pad 0");
        let video_sink_1 = scene.request_pad_simple("video_sink_%u")
            .expect("Could not get request pad 1");

        scene.release_request_pad(&video_sink_0);
        scene.release_request_pad(&video_sink_1);
    }

    #[test]
    fn test_audio_sink_pad_request() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_sink_0 = scene.request_pad_simple("audio_sink_%u");
        let audio_sink_1 = scene.request_pad_simple("audio_sink_%u");

        assert!(match audio_sink_0 {
            Some(pad) => pad.name() == "audio_sink_0",
            None => false
        });
        assert!(match audio_sink_1 {
            Some(pad) => pad.name() == "audio_sink_1",
            None => false
        });
    }

    #[test]
    fn test_audio_sink_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_sink_0 = scene.request_pad_simple("audio_sink_%u")
            .expect("Could not get request pad 0");
        let audio_sink_1 = scene.request_pad_simple("audio_sink_%u")
            .expect("Could not get request pad 1");

        scene.release_request_pad(&audio_sink_0);
        scene.release_request_pad(&audio_sink_1);
    }

    #[test]
    fn test_video_src_pad_request() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let video_src_0 = scene.request_pad_simple("video_src_%u");
        let video_src_1 = scene.request_pad_simple("video_src_%u");

        assert!(match video_src_0 {
            Some(pad) => pad.name() == "video_src_0",
            None => false
        });
        assert!(match video_src_1 {
            Some(pad) => pad.name() == "video_src_1",
            None => false
        });
    }

    #[test]
    fn test_video_src_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let video_src_0 = scene.request_pad_simple("video_src_%u")
            .expect("Could not get request pad 0");
        let video_src_1 = scene.request_pad_simple("video_src_%u")
            .expect("Could not get request pad 1");

        scene.release_request_pad(&video_src_0);
        scene.release_request_pad(&video_src_1);
    }

    #[test]
    fn test_audio_src_pad_request() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_src_0 = scene.request_pad_simple("audio_src_%u");
        let audio_src_1 = scene.request_pad_simple("audio_src_%u");

        assert!(match audio_src_0 {
            Some(pad) => pad.name() == "audio_src_0",
            None => false
        });
        assert!(match audio_src_1 {
            Some(pad) => pad.name() == "audio_src_1",
            None => false
        });
    }

    #[test]
    fn test_audio_src_pad_release() {
        set_up();

        let scene = gst::ElementFactory::make("dkcscene", Some("scene"))
            .expect("Could not make dkcscene element");
        let audio_src_0 = scene.request_pad_simple("audio_src_%u")
            .expect("Could not get request pad 0");
        let audio_src_1 = scene.request_pad_simple("audio_src_%u")
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

        let _video_sink_0 = scene.request_pad_simple("video_sink_%u")
            .expect("Could not get request pad 0");

        // Those parameters have valid value types.
        assert!(scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "width".into(), (300 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "height".into(), (300 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "xpos".into(), (300 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "ypos".into(), (300 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "alpha".into(), (0.5 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "zorder".into(), (3 as u32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());

        // Those parameters have invalid value types.
        assert!(!scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "width".into(), (3.0 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(!scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "height".into(), (3.0 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(!scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "xpos".into(), (3.0 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(!scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "ypos".into(), (3.0 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(!scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "alpha".into(), (300 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(!scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "zorder".into(), (3.0 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());

        // // This parameter does not exist.
        assert!(!scene.emit_by_name_with_values("update-input", &["video_sink_0".into(), "what".into(), (3.0 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());

        // /* Testing audio pad */

        let _audio_sink_0 = scene.request_pad_simple("audio_sink_%u")
            .expect("Could not get request pad 0");

        // // Those parameters have valid value types.
        assert!(scene.emit_by_name_with_values("update-input", &["audio_sink_0".into(), "mute".into(), true.to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(scene.emit_by_name_with_values("update-input", &["audio_sink_0".into(), "mute".into(), false.to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(scene.emit_by_name_with_values("update-input", &["audio_sink_0".into(), "volume".into(), (0.5 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());

        // Those parameters have invalid value types.
        assert!(!scene.emit_by_name_with_values("update-input", &["audio_sink_0".into(), "emit-signals".into(), (1 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(!scene.emit_by_name_with_values("update-input", &["audio_sink_0".into(), "mute".into(), (1 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
        assert!(!scene.emit_by_name_with_values("update-input", &["audio_sink_0".into(), "volume".into(), (1 as i32).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());

        // // This parameter does not exist.
        assert!(!scene.emit_by_name_with_values("update-input", &["audio_sink_0".into(), "what".into(), (3.0 as f64).to_variant().to_value()])
                .unwrap().get::<bool>().unwrap());
    }

}
