use crate::behavior::*;

use std::cell::RefCell;
use std::rc::*;

use wasmuri_container::*;

pub struct TextEditField {

    render_behavior: Rc<RefCell<dyn ComponentBehavior>>,
    render_controller: Rc<RefCell<EditTextRenderController>>
}

impl TextEditField {

    pub fn new(render_helper: (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<EditTextRenderController>>)) -> TextEditField {
        TextEditField {
            render_behavior: render_helper.0,
            render_controller: render_helper.1
        }
    }

    pub fn celled(render_helper: (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<EditTextRenderController>>)) -> Rc<RefCell<TextEditField>> {
        Rc::new(RefCell::new(Self::new(render_helper)))
    }

    pub fn get_current_text(&self) -> String {
        self.render_controller.borrow().get_current_text().to_string()
    }
}

impl Component for TextEditField {

    fn create_behaviors(&mut self) -> Vec<Rc<RefCell<dyn ComponentBehavior>>> {
        vec![Rc::clone(&self.render_behavior)]
    }
}