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

fn plugin_init(plugin: &gst::Plugin) -> bool {
    source::register(plugin);
    true
}

pub fn hello_world() {
  println!("HELLO THERE");
}
