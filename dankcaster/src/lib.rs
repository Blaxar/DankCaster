extern crate gstreamer as gst;

use gst::prelude::*;
use std::error;
use std::fmt;

#[derive(Debug)]
struct Error {
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DankCaster-related error")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "DankCaster-related error"
    }
}

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
    width: u16,
    height: u16,
    sources: Vec<Source>,
    scenes: Vec<Scene>,
    wrapped_sources: Vec<WrappedSource<'a>>,
    sinks: Vec<Sink>,
}

fn init() -> Result<(), Error> {
    unimplemented!();
}

fn terminate() -> Result<(), Error> {
    unimplemented!();
}

fn make_app<'a>(width : u16, height: u16) -> Result<App<'a>, Error> {

    Ok(App { width, height,
             sources: Vec::new(),
             scenes: Vec::new(),
             wrapped_sources: Vec::new(),
             sinks: Vec::new() })

}

impl App<'_> {
    fn make_source(self: &Self,
                   source_type: &str,
                   name: Option<&str>) -> Result<Source, Error> {

        match gst::ElementFactory::make(&format!("dkc{}source", source_type),
                                        name) {
            Some(element) => Ok(Source { element }),
            None => Err(Error {} ),
        }

    }

    fn make_sink(self: &Self,
                 sink_type: &str,
                 name: Option<&str>) -> Result<Sink, Error> {

        match gst::ElementFactory::make(&format!("dkc{}sink", sink_type),
                                        name) {
            Some(element) => Ok(Sink { element }),
            None => Err(Error {} ),
        }

    }

    fn make_scene(self: &Self, name: Option<&str>) -> Result<Scene, Error> {

        match gst::ElementFactory::make("dkcscene", name) {
            Some(element) => Ok(Scene { element }),
            None => Err(Error {} ),
        }

    }
}

impl Scene {
    fn add_source(self: &Self, source: &Source)
                  -> Result<WrappedSource, Error> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up() {

        use std::sync::{Once, ONCE_INIT};
        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| {
            gst::init().unwrap();
        });

    }

    fn tear_down() {
        unimplemented!();
    }

    #[test]
    fn test_make_app() {

        set_up();

        assert!(
            match make_app(1280, 720) {
                Ok(el) => true,
                Err(err) => false
            }
        );

    }

    #[test]
    fn test_make_source() {

        set_up();

        let app = make_app(1280, 720).unwrap();

        assert!(
            match app.make_source("dummy", None) {
                Ok(el) => true,
                Err(err) => false
            }
        );

        assert!(
            match app.make_source("IdoNotExist", None) {
                Ok(el) => false,
                Err(err) => true
            }
        );

    }

    #[test]
    fn test_make_sink() {

        set_up();

        let app = make_app(1280, 720).unwrap();

        assert!(
            match app.make_sink("dummy", None) {
                Ok(el) => true,
                Err(err) => false
            }
        );

        assert!(
            match app.make_sink("IdoNotExist", None) {
                Ok(el) => false,
                Err(err) => true
            }
        );

    }

}
