use gst::glib;

mod source;
mod sink;
mod scene;

fn plugin_init(plugin: &gst::Plugin)  -> Result<(), glib::BoolError> {
    source::register(plugin)?;
    sink::register(plugin)?;
    scene::register(plugin)?;
    Ok(())
}

gst::plugin_define!(
    dkc,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    env!("CARGO_PKG_VERSION"),
    "MIT/X11",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    "2018-11-17"
);
