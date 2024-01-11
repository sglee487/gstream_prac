mod basic_tutorial_1;
mod basic_tutorial_2;

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    basic_tutorial_1::tutorial_main();
    // basic_tutorial_2::tutorial_main();
}
