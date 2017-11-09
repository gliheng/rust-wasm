use stdweb::web::{
    IEventTarget,
    document,
    window
};

use stdweb::web::event::{
    ClickEvent,
};

pub fn main() {
    let btn = document().query_selector("#app1 button").unwrap();
    btn.add_event_listener( move |_: ClickEvent| {
        window().alert("Hello, world!");
    });
}
