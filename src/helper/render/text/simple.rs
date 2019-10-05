use std::rc::Rc;

use wasmuri_container::Cursor;
use wasmuri_container::layer::{
    ComponentAgent,
    LayerAgent,
    RenderPhase,
    RenderTrigger,
    Region
};
use wasmuri_container::params::*;

use wasmuri_core::util::{
    Color,
    TextColors
};

use wasmuri_text::{
    Font,
    TextModel
};

use super::TextRenderHelper;

pub struct SimpleTextRenderHelper {

    max_region: Region,

    text_model: TextModel,

    colors: TextColors
}

impl SimpleTextRenderHelper {

    pub fn new(text: &str, font: &Rc<Font>, max_region: Region, colors: TextColors) -> SimpleTextRenderHelper {
        SimpleTextRenderHelper {
            max_region,
            text_model: Rc::clone(font).create_text_model(text),

            colors
        }
    }

    pub fn boxed(text: &str, font: &Rc<Font>, max_region: Region, colors: TextColors) -> Box<SimpleTextRenderHelper> {
        Box::new(Self::new(text, font, max_region, colors))
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

    fn attach(&self, agent: &mut LayerAgent) -> Result<(),()> {
        agent.claim_render_space(self.max_region, RenderTrigger::Request, RenderPhase::Text)
    }

    fn get_max_region(&self) -> Region {
        self.max_region
    }

    fn get_current_region(&self) -> Region {
        let preferred_scale_y = self.max_region.get_height();
        let preferred_scale_x = self.text_model.get_render_width(preferred_scale_y);
        let scale_x;
        let scale_y;
        if preferred_scale_x <= self.max_region.get_width() {
            scale_x = preferred_scale_x;
            scale_y = preferred_scale_y;
        } else {
            scale_x = self.max_region.get_width();
            scale_y = preferred_scale_y * self.max_region.get_width() / preferred_scale_x;
        }
        let offset_x = self.max_region.get_min_x() + (self.max_region.get_width() - scale_x) / 2.0;
        let offset_y = self.max_region.get_min_y() + (self.max_region.get_height() - scale_y) / 2.0;
        return Region::new(offset_x, offset_y, offset_x + scale_x, offset_y + scale_y);
    }

    fn set_text(&mut self, new_text: &str, font: Rc<Font>, agent: &mut ComponentAgent){
        self.text_model = font.create_text_model(new_text);
        agent.request_render();
    }

    fn render(&self, _params: &mut RenderParams) -> Option<Cursor> {
        let region = self.get_current_region();
        self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), self.colors);
        wasmuri_core::util::print(&format!("Current region is {:?}", region));
        None
    }

    fn get_cursor(&self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn on_mouse_move(&self, _params: &mut MouseMoveParams) {}
}