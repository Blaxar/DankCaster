use std::sync::Once;
use gst;
use gst::prelude::*;

static mut ret: bool = false;
static START: Once = Once::new();

pub fn load() -> bool{
    unsafe {
        START.call_once(|| {
            gst::init().unwrap();
            let plugin = gst::Plugin::load_by_name("gstdkcplugin");
            match plugin {
                Some(pl) => {ret = true},
                None => {
                    let mut plugin_path = std::env::current_dir().unwrap();
                    plugin_path.push("..");

                    let registry = gst::Registry::get();

                    if registry.scan_path(plugin_path.as_path()) {
                        let plugin = gst::Plugin::load_by_name("gstdkcplugin");
                        match plugin{
                            Some(pl) => ret = true,
                            _ => ()
                        };
                    }
                }
            };
        });
        ret
    }
}
