use crate::behavior::render::TextRenderController;

use std::cell::RefCell;
use std::rc::*;

use wasmuri_container::*;

pub struct PassiveText {

    render_behavior: Rc<RefCell<dyn ComponentBehavior>>,
    render_controller: Rc<RefCell<dyn TextRenderController>>
}

impl Component for PassiveText {

    fn create_behaviors(&mut self) -> Vec<Rc<RefCell<dyn ComponentBehavior>>> {
        vec![Rc::clone(&self.render_behavior)]
    }
}

impl PassiveText {

    pub fn new(render_behavior: Rc<RefCell<dyn ComponentBehavior>>, render_controller: Rc<RefCell<dyn TextRenderController>>) -> PassiveText {
        PassiveText {
            render_behavior,
            render_controller
        }
    }

    pub fn boxed(render_behavior: Rc<RefCell<dyn ComponentBehavior>>, render_controller: Rc<RefCell<dyn TextRenderController>>) -> Box<PassiveText> {
        Box::new(Self::new(render_behavior, render_controller))
    }

    pub fn celled(render_behavior: Rc<RefCell<dyn ComponentBehavior>>, render_controller: Rc<RefCell<dyn TextRenderController>>) -> Rc<RefCell<PassiveText>> {
        Rc::new(RefCell::new(Self::new(render_behavior, render_controller)))
    }

    pub fn get_controller(&self) -> Rc<RefCell<dyn TextRenderController>> {
        Rc::clone(&self.render_controller)
    }
}