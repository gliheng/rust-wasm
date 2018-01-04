use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Gallery {
    pub urls: Vec<String>
}

js_deserializable!( Gallery );

