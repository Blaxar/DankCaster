extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().unwrap();

    let elem = gst::ElementFactory::make("videotestsrc", "source")
                       .expect("Could not create source element.");
    let pad = elem.get_static_pad("src").unwrap();
    println!("{:?}", pad);
    let ghost_pad = gst::GhostPad::new("src", &pad).unwrap();

    let source = gst::ElementFactory::make("videotestsrc", "source")
        .expect("Could not create source element.");
    let sink = gst::ElementFactory::make("xvimagesink", "sink")
        .expect("Could not create sink element.");
    let pipeline = gst::Pipeline::new("my-pipeline");

    pipeline.add_many(&[&source,&sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");
    sink.set_property_from_str("sync", "false");

    let ret = pipeline.set_state(gst::State::Playing);
    assert!(ret.is_ok(), "Unable to set the pipeline to the playing state");

    let bus = pipeline.get_bus().unwrap();
    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error()
                );
                eprintln!("Debugging information: {:?}", err.get_debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert!(ret.is_ok(), "Unable to set the pipeline to the Null state.");
}
