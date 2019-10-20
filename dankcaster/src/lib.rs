extern crate gstreamer as gst;
use gst::prelude::*;

struct Source {
    element: gst::Element,
}

struct WrappedSource<'a> {
    source: &'a gst::Element,
    element: gst::Element,
}

struct Scene {
    element: gst::Element,
}

struct Sink {
    element: gst::Element,
}

struct App<'a> {
    sources: Vec<Sink>,
    scenes: Vec<Scene>,
    wrapped_sources: Vec<WrappedSource<'a>>,
    sinks: Vec<Sink>,
}
