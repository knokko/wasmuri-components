mod simple;
mod button;
mod edit;
mod location;

pub use simple::*;
pub use button::*;
pub use edit::*;
pub use location::*;

use std::rc::*;

use wasmuri_container::*;
use wasmuri_core::util::Region;
use wasmuri_text::TextModel;

pub trait TextRenderController {

    fn set_text_model(&mut self, new_text: TextModel);

    fn set_text(&mut self, new_text: &str);

    fn get_max_region(&self) -> Region;

    fn get_current_region(&self) -> Region;
}