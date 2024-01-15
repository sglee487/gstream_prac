use std::{io, io::Write};

use gstreamer::prelude::*;

struct CustomData {
    /// Our one and only element
    playbin: gstreamer::Element,
    /// Are we in the PLAYING state?
    playing: bool,
    /// Should we terminate execution?
    terminate: bool,
    /// Is seeking enabled for this media?
    seek_enabled: bool,
    /// Have we performed the seek already?
    seek_done: bool,
    /// How long does this media last, in nanoseconds
    duration: Option<gstreamer::ClockTime>,
}

pub fn tutorial_main() {
    // Initialize GStreamer
    gstreamer::init().unwrap();

    let uri = "https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm";

    // Creat the playbin element
    let playbin = gstreamer::ElementFactory::make("playbin")
        .name("playbin")
        // Set the URI to play
        .property("uri", uri)
        .build()
        .expect("Failed to create playbin element");

    // Start playing
    playbin
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the playbin to the `Playing` state");

    // Listen to the bus
    let bus = playbin.bus().unwrap();
    let mut custom_data = CustomData {
        playbin,
        playing: false,
        terminate: false,
        seek_enabled: false,
        seek_done: false,
        duration: gstreamer::ClockTime::NONE,
    };

    while !custom_data.terminate {
        let msg = bus.timed_pop(100 * gstreamer::ClockTime::MSECOND);

        match msg {
            Some(msg) => {
                handle_message(&mut custom_data, &msg);
            }
            None => {
                if custom_data.playing {
                    let position = custom_data
                        .playbin
                        .query_position::<gstreamer::ClockTime>()
                        .expect("Could not query current position.");

                    // If we didn't know it yet, query the stream duration
                    if custom_data.duration == gstreamer::ClockTime::NONE {
                        custom_data.duration = custom_data.playbin.query_duration();
                    }

                    // Print current position and total duration
                    print!(
                        "\rPosition {} / {}",
                        position,
                        custom_data.duration.display()
                    );
                    io::stdout().flush().unwrap();

                    if custom_data.seek_enabled
                        && !custom_data.seek_done
                        && position > 10 * gstreamer::ClockTime::SECOND
                    {
                        println!("\nReached 10s, performing seek...");
                        custom_data
                            .playbin
                            .seek_simple(
                                gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT,
                                30 * gstreamer::ClockTime::SECOND,
                            )
                            .expect("Failed to seek.");
                        custom_data.seek_done = true;
                    }
                }
            }
        }
    }

    // Shutdown pipeline
    custom_data
        .playbin
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the playbin to the `Null` state");
}

fn handle_message(custom_data: &mut CustomData, msg: &gstreamer::Message) {
    use gstreamer::MessageView;

    match msg.view() {
        MessageView::Error(err) => {
            println!(
                "Error received from element {:?}: {} ({:?})",
                err.src().map(|s| s.path_string()),
                err.error(),
                err.debug()
            );
            custom_data.terminate = true;
        }
        MessageView::Eos(..) => {
            println!("End-Of-Stream reached.");
            custom_data.terminate = true;
        }
        MessageView::DurationChanged(_) => {
            // The duration has changed, mark the current one as invalid
            custom_data.duration = gstreamer::ClockTime::NONE;
        }
        MessageView::StateChanged(state_changed) => {
            if state_changed
                .src()
                .map(|s| s == &custom_data.playbin)
                .unwrap_or(false)
            {
                let new_state = state_changed.current();
                let old_state = state_changed.old();

                println!("Pipeline state changed from {old_state:?} to {new_state:?}");

                custom_data.playing = new_state == gstreamer::State::Playing;
                if custom_data.playing {
                    let mut seeking = gstreamer::query::Seeking::new(gstreamer::Format::Time);
                    if custom_data.playbin.query(&mut seeking) {
                        let (seekable, start, end) = seeking.result();
                        custom_data.seek_enabled = seekable;
                        if seekable {
                            println!("Seeking is ENABLED from {start} to {end}")
                        } else {
                            println!("Seeking is DISABLED for this stream.")
                        }
                    } else {
                        eprintln!("Seeking query failed.")
                    }
                }
            }
        }
        _ => (),
    }
}
