mod simple;
mod button;

pub use simple::*;
pub use button::*;

use std::rc::Rc;

use wasmuri_container::Cursor;
use wasmuri_container::layer::{
    LayerAgent,
    ComponentAgent,
    Region
};
use wasmuri_container::params::*;
use wasmuri_text::Font;

pub trait TextRenderHelper {

    fn attach(&self, agent: &mut LayerAgent) -> Result<(),()>;

    fn get_max_region(&self) -> Region;

    fn get_current_region(&self) -> Region;

    fn set_text(&mut self, new_text: &str, font: Rc<Font>, agent: &mut ComponentAgent);

    fn render(&self, params: &mut RenderParams) -> Option<Cursor>;

    fn get_cursor(&self, params: &mut CursorParams) -> Option<Cursor>;

    fn on_mouse_move(&self, params: &mut MouseMoveParams);
}