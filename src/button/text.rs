use crate::helper::render::text::TextRenderHelper;

use std::cell::RefCell;
use std::rc::*;

use std::convert::AsMut;

use wasmuri_container::*;
use wasmuri_container::layer::*;
use wasmuri_container::params::*;

pub struct TextButton {

    render_helper: Rc<RefCell<dyn TextRenderHelper>>,
    on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderHelper, &mut MouseClickParams)>
}

impl TextButton {

    pub fn new(render_helper: Rc<RefCell<dyn TextRenderHelper>>, on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderHelper, &mut MouseClickParams)>) -> TextButton {
        TextButton {
            render_helper,
            on_click
        }
    }

    pub fn boxed(render_helper: Rc<RefCell<dyn TextRenderHelper>>, on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderHelper, &mut MouseClickParams)>) -> Box<TextButton> {
        Box::new(TextButton::new(render_helper, on_click))
    }

    pub fn celled(render_helper: Rc<RefCell<dyn TextRenderHelper>>, on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderHelper, &mut MouseClickParams)>) -> Rc<RefCell<TextButton>> {
        Rc::new(RefCell::new(TextButton::new(render_helper, on_click)))
    }
}

impl Component for TextButton {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.borrow_mut().attach(agent).expect("Should have render space for TextButton");
        agent.claim_mouse_click_space(self.render_helper.borrow().get_max_region()).expect("Should have click space for TextButton");
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.render_helper.borrow_mut().set_agent(agent);
    }

    fn get_agent(&self) -> Weak<RefCell<ComponentAgent>> {
        Weak::clone(self.render_helper.borrow().get_agent())
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams){
        let mut render_helper = self.render_helper.borrow_mut();
        render_helper.on_click(params);
        if render_helper.get_current_region().is_inside(params.manager.get_mouse_position()) {
            let agent_cell = render_helper.get_agent().upgrade().expect("Agent shouldn't have been dropped");
            self.on_click.as_mut()(&mut agent_cell.borrow_mut(), &mut *render_helper, params);
        }
    }

    fn mouse_move(&mut self, params: &mut MouseMoveParams){
        self.render_helper.borrow_mut().on_mouse_move(params);
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor> {
        self.render_helper.borrow_mut().render(params)
    }

    fn get_cursor(&mut self, params: &mut CursorParams) -> Option<Cursor> {
        self.render_helper.borrow().get_cursor(params)
    }
}