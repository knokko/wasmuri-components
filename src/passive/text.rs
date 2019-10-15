use crate::helper::render::text::TextRenderHelper;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_container::Component;
use wasmuri_container::Cursor;
use wasmuri_container::layer::LayerAgent;
use wasmuri_container::params::*;

pub struct PassiveText {

    render_helper: Box<dyn TextRenderHelper>
}

impl Component for PassiveText {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.attach(agent).expect("Space should be free");
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor> {
        self.render_helper.render(params);
        None
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams){
        self.render_helper.on_click(params);
    }
}

impl PassiveText {

    pub fn new(render_helper: Box<dyn TextRenderHelper>) -> PassiveText {
        PassiveText {
            render_helper
        }
    }

    pub fn boxed(render_helper: Box<dyn TextRenderHelper>) -> Box<PassiveText> {
        Box::new(Self::new(render_helper))
    }

    pub fn celled(render_helper: Box<dyn TextRenderHelper>) -> Rc<RefCell<PassiveText>> {
        Rc::new(RefCell::new(Self::new(render_helper)))
    }

    pub fn set_text(&mut self, new_text: &str){
        
    }
}