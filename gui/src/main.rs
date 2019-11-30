extern crate gstreamer as gst;
use gst::prelude::*;
extern crate dankcaster as dkc;

fn main() {
    dkc::init().unwrap();

    let mut app = dkc::make_app(Some("test"), 1280, 720).unwrap();
    let dummy_source = app.make_source("dummy", Some("my dummy source")).unwrap();
    let dummy_sink = app.make_sink("dummy", Some("my dummy sink")).unwrap();
    let scene = app.make_scene(Some("my scene")).unwrap();
    let wrapped_dummy_source = scene.upgrade().unwrap().add_source(dummy_source);

    dkc::terminate();
}
