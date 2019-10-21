use crate::helper::render::text::TextRenderHelper;

use std::cell::RefCell;
use std::rc::*;

use wasmuri_container::*;
use wasmuri_container::layer::*;
use wasmuri_container::params::*;

pub struct PassiveText {

    render_helper: Rc<RefCell<dyn TextRenderHelper>>
}

impl Component for PassiveText {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.borrow_mut().attach(agent).expect("Space should be free");
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.render_helper.borrow_mut().set_agent(agent);
    }

    fn get_agent(&self) -> Weak<RefCell<ComponentAgent>> {
        Weak::clone(self.render_helper.borrow().get_agent())
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor> {
        self.render_helper.borrow().render(params);
        None
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams){
        self.render_helper.borrow_mut().on_click(params);
    }
}

impl PassiveText {

    pub fn new(render_helper: Rc<RefCell<dyn TextRenderHelper>>) -> PassiveText {
        PassiveText {
            render_helper
        }
    }

    pub fn boxed(render_helper: Rc<RefCell<dyn TextRenderHelper>>) -> Box<PassiveText> {
        Box::new(Self::new(render_helper))
    }

    pub fn celled(render_helper: Rc<RefCell<dyn TextRenderHelper>>) -> Rc<RefCell<PassiveText>> {
        Rc::new(RefCell::new(Self::new(render_helper)))
    }
}