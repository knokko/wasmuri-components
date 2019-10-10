mod simple;
mod button;
mod edit;
mod location;

pub use simple::*;
pub use button::*;
pub use edit::*;
pub use location::*;

use std::rc::Rc;

use wasmuri_container::Cursor;
use wasmuri_container::layer::{
    LayerAgent,
    ComponentAgent
};
use wasmuri_container::params::*;
use wasmuri_core::util::Region;
use wasmuri_text::Font;

pub trait TextRenderHelper {

    fn attach(&mut self, agent: &mut LayerAgent) -> Result<(),()>;

    fn get_max_region(&self) -> Region;

    fn get_current_region(&self) -> Region;

    fn set_text(&mut self, new_text: &str, font: Rc<Font>, agent: &mut ComponentAgent);

    fn render(&self, params: &mut RenderParams) -> Option<Cursor>;

    fn get_cursor(&self, params: &mut CursorParams) -> Option<Cursor>;

    fn on_mouse_move(&mut self, params: &mut MouseMoveParams);

    fn on_click(&mut self, params: &mut MouseClickParams);
}