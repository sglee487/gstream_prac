use gstreamer::prelude::*;

fn print_caps(caps: &gstreamer::CapsRef, prefix: &str) {
    if caps.is_any() {
        println!("{prefix}ANY");
        return;
    }

    if caps.is_empty() {
        println!("{prefix}EMPTY");
        return;
    }

    for structure in caps.iter() {
        println!("{}{}", prefix, structure.name());
        for (field, value) in structure.iter() {
            println!(
                "{}  {}:{}",
                prefix,
                field,
                value.serialize().unwrap().as_str()
            );
        }
    }
}

// Prints information about a Pad Template, including its Capabilitites
fn print_pad_template_information(factory: &gstreamer::ElementFactory) {
    let long_name = factory
        .metadata("long-name")
        .expect("Failed to get long-name of element factory.");
    println!("Pad Template for {long_name}:");

    if factory.num_pad_templates() == 0u32 {
        println!("  None");
        return;
    }

    for pad_template in factory.static_pad_templates() {
        if pad_template.direction() == gstreamer::PadDirection::Src {
            println!("  SRC template: '{}'", pad_template.name_template());
        } else if pad_template.direction() == gstreamer::PadDirection::Sink {
            println!("  SINK template: '{}'", pad_template.name_template());
        } else {
            println!("  UNKNOWN!!! template: '{}'", pad_template.name_template());
        }

        if pad_template.presence() == gstreamer::PadPresence::Always {
            println!("  Availability: Always");
        } else if pad_template.presence() == gstreamer::PadPresence::Sometimes {
            println!("  Availability: Sometimes");
        } else if pad_template.presence() == gstreamer::PadPresence::Request {
            println!("  Availability: On request");
        } else {
            println!("  Availability: UNKNOWN!!!");
        }

        let caps = pad_template.caps();
        println!("  Capabilities:");
        print_caps(&caps, "    ");
    }
}

fn print_pad_capabilities(element: &gstreamer::Element, pad_name: &str) {
    let pad = element
        .static_pad(pad_name)
        .expect("Could not retrieve pad");

    println!("Caps for the {pad_name} pad:");
    let caps = pad.current_caps().unwrap_or_else(|| pad.query_caps(None));
    print_caps(&caps, "      ");
}

pub fn tutorial_main() {
    // Initialize GStreamer
    gstreamer::init().unwrap();

    // Create the element factories
    let source_factory =
        gstreamer::ElementFactory::find("audiotestsrc").expect("Failed to create audiotestsrc factory.");
    let sink_factory = gstreamer::ElementFactory::find("autoaudiosink")
        .expect("Failed to create autoaudiosink factory.");

    // Print information about the pad templates of these factories
    print_pad_template_information(&source_factory);
    print_pad_template_information(&sink_factory);

    // Ask the factories to instantiate actual elements
    let source = source_factory
        .create()
        .name("source")
        .build()
        .expect("Failed to create source element");
    let sink = sink_factory
        .create()
        .name("sink")
        .build()
        .expect("Failed to create sink element");

    // Create the empty pipeline
    let pipeline = gstreamer::Pipeline::with_name("test-pipeline");

    pipeline.add_many([&source, &sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");

    // Print initial negotiated caps (in NULL state)
    println!("In NULL state:");
    print_pad_capabilities(&sink, "sink");

    // Start playing
    let res = pipeline.set_state(gstreamer::State::Playing);
    if res.is_err() {
        eprintln!(
            "Unable to set the pipeline to the `Playing` state (check the bus for error messages)."
        )
    }

    // Wait until error, EOS or State Change
    let bus = pipeline.bus().unwrap();

    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                println!(
                    "Error received from element {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            MessageView::Eos(..) => {
                println!("End-Of-Stream reached.");
                break;
            }
            MessageView::StateChanged(state_changed) =>
            // We are only interested in state-changed messages from the pipeline
                {
                    if state_changed.src().map(|s| s == &pipeline).unwrap_or(false) {
                        let new_state = state_changed.current();
                        let old_state = state_changed.old();

                        println!("Pipeline state changed from {old_state:?} to {new_state:?}");
                        print_pad_capabilities(&sink, "sink");
                    }
                }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
