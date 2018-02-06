use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GalleryItem {
    pub url: String,
    pub preview: String,
}

#[derive(Deserialize, Debug)]
pub struct Gallery {
    pub pics: Vec<GalleryItem>,
}

js_deserializable!( Gallery );

