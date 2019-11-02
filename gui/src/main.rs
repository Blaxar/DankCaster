extern crate gstreamer as gst;
use gst::prelude::*;
extern crate dankcaster as dkc;

fn main() {
    dkc::init().unwrap();

    let app = dkc::App::make(1280, 720).unwrap();
    let dummy_source = app.make_source("dummy", "my dummy source");
    let dummy_sink = app.make_sink("dummy", "my dummy sink");
    let scene = app.make_scene("my scene");
    let wrapped_dummy_source = scene.add_source(dummy_source);

    dkc::terminate();
}
