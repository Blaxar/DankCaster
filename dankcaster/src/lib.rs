extern crate gstreamer as gst;

use gst::prelude::*;
use std::error;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Error {
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

pub struct Source {
    app: Rc<AppImpl>,
    element: gst::Element,
    id: usize,
}

pub struct WrappedSource {
    source: Rc<Source>
}

pub struct Scene {
    app: Rc<AppImpl>,
    wrapped_sources: RefCell<Vec<Rc<WrappedSource>>>,
    id: usize,
}

pub struct Sink {
    app: Rc<AppImpl>,
    element: gst::Element,
    id: usize,
}

pub struct App {
    app: Rc<AppImpl>,
}

struct AppImpl {
    width: u16,
    height: u16,
    gst_bin: gst::Pipeline,
    gst_scene: gst::Element,
    sources: RefCell<Vec<Rc<Source>>>,
    scenes: RefCell<Vec<Rc<Scene>>>,
    sinks: RefCell<Vec<Rc<Sink>>>,
}

pub fn init() -> Result<(), Error> {
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

pub fn terminate() -> Result<(), Error> {
    unimplemented!();
}

pub fn make_app(name: Option<&str>, width : u16, height: u16) -> Result<App, Error> {

    let app = Rc::new( AppImpl { width, height,
                                 gst_bin: gst::Pipeline::new(name),
                                 gst_scene: gst::ElementFactory::make("dkcscene", name).unwrap(),
                                 sources: RefCell::new(vec![]),
                                 scenes: RefCell::new(vec![]),
                                 sinks: RefCell::new(vec![])});

    app.gst_bin.add(&app.gst_scene);
    Ok(App { app })

}

impl App {
    pub fn make_source(self: &mut Self,
                       source_type: &str,
                       name: Option<&str>) -> Result<Rc<Source>, Error> {

        match gst::ElementFactory::make(&format!("dkc{}source", source_type),
                                        name) {
            Some(element) => {
                let id = self.app.sources.borrow_mut().len();

                let source = Rc::new(
                    Source { app: self.app.clone(), element, id });

                self.app.sources.borrow_mut().push(source.clone());

                if self.app.gst_bin.add(&source.element).is_err() {
                    return Err(Error {});
                }

                let video_ret : std::result::Result<(), Error> = match source.element.get_static_pad("video_src") {
                    Some(video_src_pad) => {
                        match self.app.gst_scene.get_request_pad("video_sink_%u") {
                            Some(video_sink_pad) => {
                                match video_src_pad.link(&video_sink_pad) {
                                    Ok(_success) => Ok(()),
                                    Err(_error) => Err(Error {}),
                                }
                            },
                            None => Err(Error {})
                        }
                    },
                    None => Ok(()),
                };

                if video_ret.is_err() {
                    return Err(Error {});
                }

                let audio_ret : std::result::Result<(), Error> = match source.element.get_static_pad("audio_src") {
                    Some(audio_src_pad) => {
                        match self.app.gst_scene.get_request_pad("audio_sink_%u") {
                            Some(audio_sink_pad) => {
                                match audio_src_pad.link(&audio_sink_pad) {
                                    Ok(_success) => Ok(()),
                                    Err(_error) => Err(Error {}),
                                }
                            },
                            None => Err(Error {})
                        }
                    },
                    None => Ok(()),
                };

                if audio_ret.is_err() {
                    return Err(Error {});
                }
                            
                Ok(source)
            },
            None => Err(Error {}),
        }

    }

    pub fn make_sink(self: &mut Self,
                     sink_type: &str,
                     name: Option<&str>) -> Result<Rc<Sink>, Error> {

        match gst::ElementFactory::make(&format!("dkc{}sink", sink_type),
                                        name) {
            Some(element) => {
                let id = self.app.sinks.borrow_mut().len();

                let sink = Rc::new(
                    Sink { app: self.app.clone(), element, id });

                self.app.sinks.borrow_mut().push(sink.clone());

                if self.app.gst_bin.add(&sink.element).is_err() {
                    return Err(Error {});
                }

                let video_ret : std::result::Result<(), Error> = match sink.element.get_static_pad("video_sink") {
                    Some(video_sink_pad) => {
                        match self.app.gst_scene.get_request_pad("video_src_%u") {
                            Some(video_src_pad) => {
                                match video_src_pad.link(&video_sink_pad) {
                                    Ok(_success) => Ok(()),
                                    Err(_error) => Err(Error {}),
                                }
                            },
                            None => Err(Error {})
                        }
                    },
                    None => Ok(()),
                };

                if video_ret.is_err() {
                    return Err(Error {});
                }

                let audio_ret : std::result::Result<(), Error> = match sink.element.get_static_pad("audio_sink") {
                    Some(audio_sink_pad) => {
                        match self.app.gst_scene.get_request_pad("audio_src_%u") {
                            Some(audio_src_pad) => {
                                match audio_src_pad.link(&audio_sink_pad) {
                                    Ok(_success) => Ok(()),
                                    Err(_error) => Err(Error {}),
                                }
                            },
                            None => Err(Error {})
                        }
                    },
                    None => Ok(()),
                };

                if audio_ret.is_err() {
                    return Err(Error {});
                }

                Ok(sink)
            },
            None => Err(Error {}),
        }

    }

    pub fn make_scene(self: &Self, name: Option<&str>) -> Result<Rc<Scene>, Error> {

        let id = self.app.scenes.borrow_mut().len();

        let scene = Rc::new(
            Scene { app: self.app.clone(), wrapped_sources: RefCell::new(vec![]), id });

        self.app.scenes.borrow_mut().push(scene.clone());

        Ok(scene)

    }

    pub fn turn_on(self: &mut Self) -> Result<(), Error> {
        let pipeline = &self.app.gst_bin;

        pipeline.set_state(gst::State::Playing);

        let bus = pipeline
            .get_bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    pipeline.set_state(gst::State::Null);
                }
                _ => (),
            }
        }

        pipeline.set_state(gst::State::Null);

        Ok(())

    }
}

impl Scene {
    pub fn add_source(self: &Self, source: Rc<Source>)
                      -> Result<Rc<WrappedSource>, Error> {

        let id = self.app.sinks.borrow_mut().len();
        let wrapped_source = Rc::new(WrappedSource { source: source.clone() });
        self.wrapped_sources.borrow_mut().push(wrapped_source.clone());
        Ok(wrapped_source)

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

        let mut app = make_app(Some("test"), 1280, 720).expect("Could not make app.");

        assert_eq!(app.app.sources.borrow_mut().len(), 0);

        let source = app.make_source("dummy", None);

        assert!(
            match &source {
                Ok(el) => true,
                Err(err) => false
            }
        );

        let gst_src = &source.unwrap().element; 

        assert!(match gst_src.get_static_pad("video_src").unwrap().get_peer() {
            Some(peer_pad) => peer_pad.get_name().eq("video_sink_0"),
            None => false
        });

        assert!(match gst_src.get_static_pad("audio_src").unwrap().get_peer() {
            Some(peer_pad) => peer_pad.get_name().eq("audio_sink_0"),
            None => false
        });

        assert_eq!(app.app.sources.borrow_mut().len(), 1);

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

        let mut app = make_app(Some("test"), 1280, 720).expect("Could not make app.");

        assert_eq!(app.app.sinks.borrow_mut().len(), 0);

        let sink = app.make_sink("dummy", None);

        assert!(
            match &sink {
                Ok(el) => true,
                Err(err) => false
            }
        );

        let gst_sink = &sink.unwrap().element;

        assert!(match gst_sink.get_static_pad("video_sink").unwrap().get_peer() {
            Some(peer_pad) => peer_pad.get_name().eq("video_src_0"),
            None => false
        });

        assert!(match gst_sink.get_static_pad("audio_sink").unwrap().get_peer() {
            Some(peer_pad) => peer_pad.get_name().eq("audio_src_0"),
            None => false
        });

        assert_eq!(app.app.sinks.borrow_mut().len(), 1);

        assert!(
            match app.make_sink("IdoNotExist", None) {
                Ok(el) => false,
                Err(err) => true
            }
        );

    }

    #[test]
    fn test_make_scene() {

        set_up();

        let mut app = make_app(Some("test"), 1280, 720).expect("Could not make app.");

        let scene = app.make_scene(Some("dummyscene"));

        assert!(
            match &scene {
                Ok(el) => true,
                Err(err) => false
            }
        );

    }

    #[test]
    fn test_scene_add_source() {

        set_up();

        let mut app = make_app(Some("test"), 1280, 720).expect("Could not make app.");

        let scene = app.make_scene(Some("dummyscene")).expect("Could not make scene.");
        let source = app.make_source("dummy", None).expect("Could not make source.");

        assert!(
            match &scene.add_source(source) {
                Ok(wrpd) => true,
                Err(err) => false
            }
        );

    }

}
