use gstreamer::prelude::*;

pub fn tutorial_main() {
    // Initialize gstreamer
    gstreamer::init().unwrap();

    let uri = "http://desmottes.be/~cassidy/files/brol/test.mkv";

    // Create the elements
    let source = gstreamer::ElementFactory::make("uridecodebin")
        .name("source")
        // Set the URI to play
        .property("uri", uri)
        .build()
        .expect("Could not create uridecodebin element.");
    let convert = gstreamer::ElementFactory::make("audioconvert")
        .name("convert")
        .build()
        .expect("Could not create convert element.");
    let sink = gstreamer::ElementFactory::make("autoaudiosink")
        .name("sink")
        .build()
        .expect("Could not create sink element.");
    let resample = gstreamer::ElementFactory::make("audioresample")
        .name("resample")
        .build()
        .expect("Could not create resample element.");

    // Create the empty pipeline
    let pipeline = gstreamer::Pipeline::with_name("test-pipeline");

    // Build the pipeline Note that we are NOT linking the source at this
    // point. We will do it later.
    pipeline
        .add_many([&source, &convert, &resample, &sink])
        .unwrap();
    gstreamer::Element::link_many([&convert, &resample, &sink]).expect("Elements could not be linked.");

    // Connect the pad-added signal
    source.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());

        src.downcast_ref::<gstreamer::Bin>()
            .unwrap()
            .debug_to_dot_file_with_ts(gstreamer::DebugGraphDetails::ALL, "pad-added");

        let sink_pad = convert
            .static_pad("sink")
            .expect("Failed to get static sink pad from convert");
        if sink_pad.is_linked() {
            println!("We are already linked. Ignoring.");
            return;
        }

        let new_pad_caps = src_pad
            .current_caps()
            .expect("Failed to get caps of new pad.");
        let new_pad_struct = new_pad_caps
            .structure(0)
            .expect("Failed to get first structure of caps.");
        let new_pad_type = new_pad_struct.name();

        let is_audio = new_pad_type.starts_with("audio/x-raw");
        if !is_audio {
            println!("It has type {new_pad_type} which is not raw audio. Ignoring.");
            return;
        }

        let res = src_pad.link(&sink_pad);
        if res.is_err() {
            println!("Type is {new_pad_type} but link failed.");
        } else {
            println!("Link succeeded (type {new_pad_type}).");
        }
    });

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
                    "Error received from element {:?} {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed.src().map(|s| s == &pipeline).unwrap_or(false) {
                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current()
                    );
                }
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}