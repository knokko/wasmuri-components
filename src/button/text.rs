use crate::behavior::render::TextRenderController;
use crate::behavior::mouse::ClickActionBehavior;

use std::cell::RefCell;
use std::rc::*;

use wasmuri_container::*;
use wasmuri_container::layer::*;
use wasmuri_container::params::*;

pub struct TextButton {

    render_behavior: Rc<RefCell<dyn ComponentBehavior>>,
    render_controller: Rc<RefCell<dyn TextRenderController>>,
    click_behavior: Rc<RefCell<dyn ComponentBehavior>>
}

impl TextButton {

    pub fn new(render_behavior: Rc<RefCell<dyn ComponentBehavior>>, render_controller: Rc<RefCell<dyn TextRenderController>>,
    on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderController, &mut MouseClickParams)>) -> TextButton {
        TextButton {
            click_behavior: ClickActionBehavior::celled(on_click, Rc::clone(&render_controller)),
            render_behavior,
            render_controller
        }
    }

    pub fn celled(render_helper: (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<dyn TextRenderController>>),
    on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderController, &mut MouseClickParams)>) -> Rc<RefCell<TextButton>> {
        Rc::new(RefCell::new(TextButton::new(render_helper.0, render_helper.1, on_click)))
    }

    pub fn get_controller(&self) -> Rc<RefCell<dyn TextRenderController>> {
        Rc::clone(&self.render_controller)
    }
}

impl Component for TextButton {

    fn create_behaviors(&mut self) -> Vec<Rc<RefCell<dyn ComponentBehavior>>> {
        vec![Rc::clone(&self.render_behavior), Rc::clone(&self.click_behavior)]
    }
}