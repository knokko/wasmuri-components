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

pub struct SimpleTextRenderHelper {

    region: TextRegionProps,

    text_model: TextModel,

    colors: TextColors
}

impl SimpleTextRenderHelper {

    pub fn new(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> SimpleTextRenderHelper {
        SimpleTextRenderHelper {
            region,
            text_model: Rc::clone(font).create_text_model(text),

            colors
        }
    }

    pub fn boxed(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> Box<SimpleTextRenderHelper> {
        Box::new(Self::new(text, font, region, colors))
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

impl TextRenderHelper for SimpleTextRenderHelper {

    fn attach(&mut self, agent: &mut LayerAgent) -> Result<(),()> {
        agent.claim_render_space(self.region.get_max_region(), RenderTrigger::Request, RenderPhase::Text)
    }

    fn get_max_region(&self) -> Region {
        self.region.get_max_region()
    }

    fn get_current_region(&self) -> Region {
        self.region.get_current_region(&self.text_model)
    }

    fn set_text(&mut self, new_text: &str, font: Rc<Font>, agent: &mut ComponentAgent){
        self.text_model = font.create_text_model(new_text);
        agent.request_render();
    }

    fn render(&self, params: &mut RenderParams) -> Option<Cursor> {
        let region = self.get_current_region();
        if self.region.should_clear_remaining(&self.text_model, params) {
            self.text_model.get_font().fill_rect(self.get_max_region(), self.colors.background_color);
        }
        self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), self.colors);
        None
    }

    fn get_cursor(&self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn on_mouse_move(&mut self, _params: &mut MouseMoveParams) {}

    fn on_click(&mut self, _params: &mut MouseClickParams) {}
}