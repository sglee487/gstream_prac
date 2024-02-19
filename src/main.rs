mod basic_tutorial_1;
mod basic_tutorial_2;
mod basic_tutorial_3;
mod basic_tutorial_4;
mod basic_tutorial_6;
mod playback_tutorial_1;
mod playback_tutorial_2;
mod get_frame;
mod basic_tutorial_9;
mod basic_tutorial_8;
mod basic_tutorial_8_custom;
// mod plugin_prac;

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    basic_tutorial_1::tutorial_main();
    // basic_tutorial_2::tutorial_main();
    // basic_tutorial_3::tutorial_main();
    // basic_tutorial_4::tutorial_main();
    // basic_tutorial_6::tutorial_main();
    // basic_tutorial_8::tutorial_main();
    // basic_tutorial_8_custom::tutorial_main();
    // basic_tutorial_9::tutorial_main();

    // playback_tutorial_1::tutorial_main();
    // playback_tutorial_2::tutorial_main();

    // get_frame::main();


    // basic_tutorial_8_custom::tutorial_main();
    // basic_tutorial_2_custom2::tutorial_main();
    // plugin_prac::main();
}
