use serde::{Deserialize, Serialize};

pub mod js_tiptap;
pub mod tiptap_instance;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ImageResource {
    // Example: image.png
    pub title: String,
    // Example: "An example image, ..."
    pub alt: String,
    // Example: https:://my-site.com/public/image.png
    pub url: String,
}
