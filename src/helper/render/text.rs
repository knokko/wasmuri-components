use std::rc::Rc;

use wasmuri_container::layer::{
    ComponentAgent,
    LayerAgent,
    RenderPhase,
    RenderTrigger,
    Region
};

use wasmuri_core::util::color::Color;

use wasmuri_text::{
    Font,
    TextModel
};

pub trait TextRenderHelper {

    fn attach(&self, agent: &mut LayerAgent) -> Result<(),()>;

    fn get_max_region(&self) -> Region;

    fn get_current_region(&self) -> Region;

    fn render(&self);
}

pub struct SimpleTextRenderHelper {

    max_region: Region,

    text_model: TextModel,

    fill_color: Color,
    stroke_color: Color,
    background_color: Color
}

impl SimpleTextRenderHelper {

    pub fn new(text: &str, font: Rc<Font>, max_region: Region, fill_color: Color, stroke_color: Color, background_color: Color) -> SimpleTextRenderHelper {
        SimpleTextRenderHelper {
            max_region,
            text_model: font.create_text_model(text),

            fill_color,
            stroke_color,
            background_color
        }
    }

    pub fn set_text(&mut self, new_text: &str, font: Rc<Font>, agent: &mut ComponentAgent){
        self.text_model = font.create_text_model(new_text);
        agent.request_render();
    }

    pub fn set_fill_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.fill_color = new_color;
        agent.request_render();
    }

    pub fn set_stroke_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.stroke_color = new_color;
        agent.request_render();
    }

    pub fn set_background_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.background_color = new_color;
        agent.request_render();
    }

    pub fn set_colors(&mut self, new_fill_color: Color, new_stroke_color: Color, new_background_color: Color, agent: &mut ComponentAgent){
        self.fill_color = new_fill_color;
        self.stroke_color = new_stroke_color;
        self.background_color = new_background_color;
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

    fn render(&self){
        let region = self.get_current_region();
        self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), self.fill_color, self.stroke_color, self.background_color);
    }
}