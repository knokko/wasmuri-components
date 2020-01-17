use crate::behavior::TextRenderController;

use std::cell::RefCell;
use std::rc::*;

use std::convert::AsMut;

use wasmuri_container::*;

pub struct ClickActionBehavior {

    on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderController, &mut MouseClickParams)>,

    agent: Option<Weak<RefCell<ComponentAgent>>>,
    render_controller: Rc<RefCell<dyn TextRenderController>>
}

impl ClickActionBehavior {

    pub fn new(on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderController, &mut MouseClickParams)>,
    render_controller: Rc<RefCell<dyn TextRenderController>>) -> ClickActionBehavior {
        ClickActionBehavior {
            on_click,

            agent: None,
            render_controller
        }
    }

    pub fn celled(on_click: Box<dyn FnMut(&mut ComponentAgent, &mut dyn TextRenderController, &mut MouseClickParams)>,
    render_controller: Rc<RefCell<dyn TextRenderController>>) -> Rc<RefCell<ClickActionBehavior>> {
        Rc::new(RefCell::new(Self::new(on_click, render_controller)))
    }
}

impl ComponentBehavior for ClickActionBehavior {

    fn attach(&mut self, agent: &mut LayerAgent){
        agent.claim_mouse_click_space(self.render_controller.borrow().get_max_region()).expect("Should have click space for ClickActionBehavior");
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.agent = Some(agent);
    }

    fn get_agent(&self) -> &Weak<RefCell<ComponentAgent>> {
        self.agent.as_ref().expect("Agent should have been set by now")
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams) -> bool {
        let mut render_controller = self.render_controller.borrow_mut();
        if render_controller.get_current_region().is_float_inside(params.manager.get_mouse_position()) {
            let agent_cell = self.get_agent().upgrade().expect("Agent shouldn't have been dropped");
            self.on_click.as_mut()(&mut agent_cell.borrow_mut(), &mut *render_controller, params);
            true
        } else {
            false
        }
    }
}