use stdweb::web::{
    INode,
    document,
};

pub fn main() {
    let div = document().query_selector("#app2");
    if let Some(div) = div {
        for i in 0..5 {
            let li = document().create_element("li");
            let s = String::from("label ") + i.to_string().as_str();
            li.append_child(&document().create_text_node(&s));
            div.append_child(&li);
        }
    }
}
