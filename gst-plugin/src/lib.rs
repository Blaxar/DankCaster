extern crate glib;
extern crate gobject_subclass;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_base as gst_base;
#[macro_use]
extern crate gst_plugin;

plugin_define!(
    b"gstdkcplugin\0",
    b"DankCaster Gstreamer Plugin\0",
    plugin_init,
    b"1.0\0",
    b"MIT/X11\0",
    b"dkcgstplugin\0",
    b"dkcgstplugin\0",
    b"https://github.com/Blaxar/DankCaster\0",
    b"2018-11-17\0"
);

mod source;
mod scene;
mod sink;
mod plugin_once;

fn plugin_init(plugin: &gst::Plugin) -> bool {
    source::register(plugin);
    sink::register(plugin);
    scene::register(plugin);
    true
}

pub fn hello_world() {
  println!("HELLO THERE");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_plugin_loaded() {
        assert!(plugin_once::load());
    }
}
