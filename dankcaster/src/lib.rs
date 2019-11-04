extern crate gstreamer as gst;

use gst::prelude::*;
use std::error;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

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
    app: Rc<AppImpl>,
    element: gst::Element,
    id: usize,
}

struct WrappedSource {
    source_id: usize,
    element: gst::Element,
}

struct Scene {
    element: gst::Element,
}

struct Sink {
    app: Rc<AppImpl>,
    element: gst::Element,
    id: usize,
}

struct App {
    app: Rc<AppImpl>,
}

struct AppImpl {
    width: u16,
    height: u16,
    gst_bin: gst::Bin,
    gst_scene: gst::Element,
    sources: RefCell<Vec<Weak<Source>>>,
    scenes: RefCell<Vec<Weak<Scene>>>,
    wrapped_sources: RefCell<Vec<Weak<WrappedSource>>>,
    sinks: RefCell<Vec<Weak<Sink>>>,
}

fn init() -> Result<(), Error> {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        gst::init().unwrap();
    });

    /*
     *  TODO: Check if Dankcaster gst-plugin elements are
     *        to be found in the registry.
     */
    Ok(())
}

fn terminate() -> Result<(), Error> {
    unimplemented!();
}

fn make_app(name: Option<&str>, width : u16, height: u16) -> Result<App, Error> {

    let app = Rc::new( AppImpl { width, height,
                                 gst_bin: gst::Bin::new(name),
                                 gst_scene: gst::ElementFactory::make("dkcscene", name).unwrap(),
                                 sources: RefCell::new(vec![]),
                                 scenes: RefCell::new(vec![]),
                                 wrapped_sources: RefCell::new(vec![]),
                                 sinks: RefCell::new(vec![])});
    Ok(App { app })

}

impl App {
    fn make_source(self: &mut Self,
                   source_type: &str,
                   name: Option<&str>) -> Result<Weak<Source>, Error> {

        match gst::ElementFactory::make(&format!("dkc{}source", source_type),
                                        name) {
            Some(element) => {
                let id = self.app.sources.borrow_mut().len();

                let source = Rc::new(
                    Source { app: self.app.clone(), element, id });

                self.app.sources.borrow_mut().push(
                    Rc::downgrade(&source));

                Ok(Rc::downgrade(&source))
            },
            None => Err(Error {}),
        }

    }

    fn make_sink(self: &mut Self,
                 sink_type: &str,
                 name: Option<&str>) -> Result<Weak<Sink>, Error> {

        match gst::ElementFactory::make(&format!("dkc{}sink", sink_type),
                                        name) {
            Some(element) => {
                let id = self.app.sinks.borrow_mut().len();

                let sink = Rc::new(
                    Sink { app: self.app.clone(), element, id });

                self.app.sinks.borrow_mut().push(
                    Rc::downgrade(&sink));

                Ok(Rc::downgrade(&sink))
            },
            None => Err(Error {}),
        }

    }

    fn make_scene(self: &Self, name: Option<&str>) -> Result<Scene, Error> {

        match gst::ElementFactory::make("dkcscene", name) {
            Some(element) => Ok(Scene { element }),
            None => Err(Error {}),
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

        use std::sync::Once;
        static INIT: Once = Once::new();

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
            match make_app(Some("test"), 1280, 720) {
                Ok(el) => true,
                Err(err) => false
            }
        );

    }

    #[test]
    fn test_make_source() {

        set_up();

        let mut app = make_app(Some("test"), 1280, 720).unwrap();

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

        let mut app = make_app(Some("test"), 1280, 720).unwrap();

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
