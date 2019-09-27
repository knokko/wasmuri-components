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

pub trait TextRenderHelper {

    fn attach(&self, agent: &mut LayerAgent) -> Result<(),()>;

    fn get_max_region(&self) -> Region;

    fn get_current_region(&self) -> Region;

    fn render(&self, params: &mut RenderParams) -> Option<Cursor>;

    fn get_cursor(&self, params: &mut CursorParams) -> Option<Cursor>;

    fn on_mouse_move(&self, params: &mut MouseMoveParams);
}

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

    pub fn set_text(&mut self, new_text: &str, font: Rc<Font>, agent: &mut ComponentAgent){
        self.text_model = font.create_text_model(new_text);
        agent.request_render();
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

    fn render(&self, _params: &mut RenderParams) -> Option<Cursor> {
        let region = self.get_current_region();
        self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), self.colors);
        None
    }

    fn get_cursor(&self, _params: &mut CursorParams) -> Option<Cursor> {
        None
    }

    fn on_mouse_move(&self, _params: &mut MouseMoveParams) {}
}

pub struct ButtonTextRenderHelper {

    max_region: Region,
    text_model: TextModel,

    base_colors: TextColors,
    hover_colors: TextColors
}

fn lighten_component(component: u8) -> u8 {
    ((component as u16 + 255) / 2) as u8
}

fn lighten_color(color: Color) -> Color {
    Color::from_rgba(lighten_component(color.get_red()), lighten_component(color.get_green()), lighten_component(color.get_blue()), lighten_component(color.get_alpha()))
}

fn lighten_colors(colors: TextColors) -> TextColors {
    TextColors::new(lighten_color(colors.fill_color), lighten_color(colors.stroke_color), lighten_color(colors.background_color))
}

impl ButtonTextRenderHelper {

    pub fn new(text: &str, font: &Rc<Font>, max_region: Region, base_colors: TextColors, hover_colors: TextColors) -> ButtonTextRenderHelper {
        ButtonTextRenderHelper {
            max_region,
            text_model: Rc::clone(font).create_text_model(text),

            base_colors,
            hover_colors
        }
    }

    pub fn simple(text: &str, font: &Rc<Font>, max_region: Region, colors: TextColors) -> ButtonTextRenderHelper {
        Self::new(text, font, max_region, colors, lighten_colors(colors))
    }

    pub fn set_text(&mut self, new_text: &str, font: Rc<Font>, agent: &mut ComponentAgent){
        self.text_model = font.create_text_model(new_text);
        agent.request_render();
    }

    pub fn set_base_fill_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.fill_color = new_color;
        agent.request_render();
    }

    pub fn set_base_stroke_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.stroke_color = new_color;
        agent.request_render();
    }

    pub fn set_base_background_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.background_color = new_color;
        agent.request_render();
    }

    pub fn set_base_colors(&mut self, new_colors: TextColors, agent: &mut ComponentAgent){
        self.base_colors = new_colors;
        agent.request_render();
    }

    pub fn set_hover_fill_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.hover_colors.fill_color = new_color;
        agent.request_render();
    }

    pub fn set_hover_stroke_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.hover_colors.stroke_color = new_color;
        agent.request_render();
    }

    pub fn set_hover_background_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.hover_colors.background_color = new_color;
        agent.request_render();
    }

    pub fn set_hover_colors(&mut self, new_colors: TextColors, agent: &mut ComponentAgent){
        self.hover_colors = new_colors;
        agent.request_render();
    }

    pub fn set_fill_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.fill_color = new_color;
        self.hover_colors.fill_color = lighten_color(new_color);
        agent.request_render();
    }

    pub fn set_stroke_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.stroke_color = new_color;
        self.hover_colors.stroke_color = lighten_color(new_color);
        agent.request_render();
    }

    pub fn set_background_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.background_color = new_color;
        self.hover_colors.background_color = lighten_color(new_color);
        agent.request_render();
    }

    pub fn set_colors(&mut self, new_colors: TextColors, agent: &mut ComponentAgent){
        self.base_colors = new_colors;
        self.hover_colors = lighten_colors(new_colors);
        agent.request_render();
    }
}

impl TextRenderHelper for ButtonTextRenderHelper {

    fn attach(&self, agent: &mut LayerAgent) -> Result<(),()> {
        agent.claim_mouse_move_space(self.max_region);
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

    fn render(&self, params: &mut RenderParams) -> Option<Cursor> {
        let region = self.get_current_region();
        if region.is_inside(params.manager.get_mouse_position()) {
            self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), self.hover_colors);
            Some(Cursor::POINTER)
        } else {
            self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), self.base_colors);
            None
        }
    }

    fn get_cursor(&self, params: &mut CursorParams) -> Option<Cursor> {
        let region = self.get_current_region();
        if region.is_inside(params.manager.get_mouse_position()) {
            Some(Cursor::POINTER)
        } else {
            None
        }
    }

    fn on_mouse_move(&self, params: &mut MouseMoveParams) {
        let region = self.get_current_region();
        let prev_mouse = params.manager.get_mouse_position();
        let next_mouse = params.manager.to_gl_coords(params.event.get_new_position());
        if region.is_inside(prev_mouse) != region.is_inside(next_mouse) {
            params.agent.request_render();
        }
    }
}