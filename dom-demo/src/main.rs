extern crate stdweb;

mod app1;
mod app2;

fn main() {
    stdweb::initialize();
    app1::main();
    app2::main();
    stdweb::event_loop();
}
