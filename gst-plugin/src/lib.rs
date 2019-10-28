
#[macro_use]
extern crate glib;
#[macro_use]
extern crate gstreamer as gst;

gst_plugin_define!(
    rstutorial,
    "DankCaster Gstreamer Plugin",
    plugin_init,
    "1.0",
    "MIT/X11",
    "dkcgstplugin",
    "dkcgstplugin",
    "https://github.com/Blaxar/DankCaster",
    "2018-11-17"
);

mod source;
mod sink;
mod scene;

fn plugin_init(plugin: &gst::Plugin)  -> Result<(), glib::BoolError> {
    source::register(plugin)?;
    sink::register(plugin)?;
    scene::register(plugin)?;
    Ok(())
}
