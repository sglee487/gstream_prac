use gstreamer::prelude::*;

pub fn tutorial_main() {
    // Initialize gstreamerreamer
    gstreamer::init().unwrap();

    // Create the elements
    let source = gstreamer::ElementFactory::make("videotestsrc")
        .name("source")
        .property_from_str("pattern", "smpte")
        .build()
        .expect("Could not create source element.");
    let sink = gstreamer::ElementFactory::make("autovideosink")
        .name("sink")
        .build()
        .expect("Could not create sink element");

    // Create the empty pipeline
    let pipeline = gstreamer::Pipeline::with_name("test-pipeline");

    // Build the pipeline
    pipeline.add_many([&source, &sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");

    // Start playing
    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}