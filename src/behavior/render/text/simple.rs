use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_container::Cursor;
use wasmuri_container::layer::{
    ComponentAgent,
    LayerAgent,
    RenderPhase,
    RenderTrigger
};
use wasmuri_container::params::*;

use wasmuri_core::color::*;

use wasmuri_text::{
    Font,
    TextModel
};

use super::*;

pub struct SimpleTextRenderController {

    region: TextRegionProps,
    agent: Option<Weak<RefCell<ComponentAgent>>>,

    text_model: TextModel,

    colors: TextColors
}

impl SimpleTextRenderController {

    pub fn new(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> SimpleTextRenderController {
        SimpleTextRenderController {
            region,
            agent: None,
            text_model: Rc::clone(font).create_text_model(text),

            colors
        }
    }

    pub fn celled(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> Rc<RefCell<SimpleTextRenderController>> {
        Rc::new(RefCell::new(Self::new(text, font, region, colors)))
    }

    pub fn tuple(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<dyn TextRenderController>>) {
        let instance = Rc::new(RefCell::new(Self::new(text, font, region, colors)));
        (Rc::clone(&instance) as Rc<RefCell<dyn ComponentBehavior>>, instance)
    }

    pub fn set_fill_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.colors.fill_color = new_color;
        agent.request_render();
    }

    pub fn set_stroke_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.colors.stroke_color = new_color;
        agent.request_render();
    }

    pub fn set_background_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.colors.background_color = new_color;
        agent.request_render();
    }

    pub fn set_colors(&mut self, new_colors: TextColors, agent: &mut ComponentAgent){
        self.colors = new_colors;
        agent.request_render();
    }
}

impl ComponentBehavior for SimpleTextRenderController {

    fn attach(&mut self, agent: &mut LayerAgent){
        agent.claim_render_space(self.region.get_max_region(), RenderTrigger::Request, RenderPhase::Text).expect("Should have render space for SimpleTextRenderHelper");
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.agent = Some(agent);
    }

    fn get_agent(&self) -> &Weak<RefCell<ComponentAgent>> {
        self.agent.as_ref().expect("Agent should have been set by now")
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor> {
        let region = self.get_current_region();
        if self.region.should_clear_remaining(&self.text_model, params) {
            self.text_model.get_font().fill_rect(self.get_max_region(), self.colors.background_color);
        }
        self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), self.colors);
        None
    }

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }
}

impl TextRenderController for SimpleTextRenderController {

    fn get_max_region(&self) -> Region {
        self.region.get_max_region()
    }

    fn get_current_region(&self) -> Region {
        self.region.get_current_region(&self.text_model)
    }

    fn set_text(&mut self, new_text: &str){
        self.text_model = Rc::clone(self.text_model.get_font()).create_text_model(new_text);
        self.agent.as_ref().expect("Agent should have been set by now").upgrade().expect("Component agent should not have been dropped").borrow_mut().request_render();
    }

    fn set_text_model(&mut self, new_text: TextModel){
        self.text_model = new_text;
        self.agent.as_ref().expect("Agent should have been set by now").upgrade().expect("Component agent should not have been dropped").borrow_mut().request_render();
    }
}