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

pub struct EditTextRenderHelper {

    region: TextRegionProps,
    text_model: TextModel,

    base_colors: TextColors,
    hover_colors: TextColors,
    active_colors: TextColors,

    active: bool
}

fn lighten_component(component: u8) -> u8 {
    ((component as u16 + 255) / 2) as u8
}

fn lighten_color(color: Color) -> Color {
    Color::from_rgba(lighten_component(color.get_red()), lighten_component(color.get_green()), lighten_component(color.get_blue()), lighten_component(color.get_alpha()))
}

fn lighten_colors(colors: TextColors) -> TextColors {
    TextColors::new(colors.fill_color, colors.stroke_color, lighten_color(colors.background_color))
}

fn darken_component(component: u8) -> u8 {
    ((component as u16 * 5) / 6) as u8
}

fn darken_color(color: Color) -> Color {
    Color::from_rgba(darken_component(color.get_red()), darken_component(color.get_green()), darken_component(color.get_blue()), color.get_alpha())
}

fn darken_colors(colors: TextColors) -> TextColors {
    TextColors::new(colors.fill_color, colors.stroke_color, darken_color(colors.background_color))
}

impl EditTextRenderHelper {

    pub fn new(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors, active_colors: TextColors) -> EditTextRenderHelper {
        EditTextRenderHelper {
            region,
            text_model: Rc::clone(font).create_text_model(text),

            base_colors,
            hover_colors,
            active_colors,

            active: false
        }
    }

    pub fn boxed(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors, active_colors: TextColors) -> Box<EditTextRenderHelper> {
        Box::new(EditTextRenderHelper::new(text, font, region, base_colors, hover_colors, active_colors))
    }

    pub fn simple(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> EditTextRenderHelper {
        Self::new(text, font, region, colors, darken_colors(colors), lighten_colors(colors))
    }

    pub fn simple_boxed(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> Box<EditTextRenderHelper> {
        Box::new(Self::simple(text, font, region, colors))
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

    pub fn set_active_fill_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.active_colors.fill_color = new_color;
        agent.request_render();
    }

    pub fn set_active_stroke_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.active_colors.stroke_color = new_color;
        agent.request_render();
    }

    pub fn set_active_background_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.active_colors.background_color = new_color;
        agent.request_render();
    }

    pub fn set_active_colors(&mut self, new_colors: TextColors, agent: &mut ComponentAgent){
        self.active_colors = new_colors;
        agent.request_render();
    }

    pub fn set_fill_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.fill_color = new_color;
        self.hover_colors.fill_color = darken_color(new_color);
        self.active_colors.fill_color = lighten_color(new_color);
        agent.request_render();
    }

    pub fn set_stroke_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.stroke_color = new_color;
        self.hover_colors.stroke_color = darken_color(new_color);
        self.active_colors.stroke_color = lighten_color(new_color);
        agent.request_render();
    }

    pub fn set_background_color(&mut self, new_color: Color, agent: &mut ComponentAgent){
        self.base_colors.background_color = new_color;
        self.hover_colors.background_color = darken_color(new_color);
        self.active_colors.background_color = lighten_color(new_color);
        agent.request_render();
    }

    pub fn set_colors(&mut self, new_colors: TextColors, agent: &mut ComponentAgent){
        self.base_colors = new_colors;
        self.hover_colors = darken_colors(new_colors);
        self.active_colors = lighten_colors(new_colors);
        agent.request_render();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, new_active: bool) {
        self.active = new_active;
    }
}

impl TextRenderHelper for EditTextRenderHelper {

    fn attach(&mut self, agent: &mut LayerAgent) -> Result<(),()> {
        agent.claim_mouse_in_out_space(self.region.get_max_region());
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
        let region = self.get_max_region();
        let colors;
        let result = match region.is_inside(params.manager.get_mouse_position()) {
            true => Some(Cursor::TEXT),
            false => None
        };
        
        if self.active {
            colors = self.active_colors;
        } else if result.is_some() {
            colors = self.hover_colors;
        } else {
            colors = self.base_colors;
        }

        if self.region.should_clear_remaining(&self.text_model, params) {
            self.text_model.get_font().fill_rect(self.region.get_max_region(), colors.background_color);
        }

        self.text_model.render(region.get_min_x(), region.get_min_y(), region.get_height(), colors);
        result
    }

    fn get_cursor(&self, params: &mut CursorParams) -> Option<Cursor> {
        let region = self.get_max_region();
        if region.is_inside(params.manager.get_mouse_position()) {
            Some(Cursor::TEXT)
        } else {
            None
        }
    }

    fn on_mouse_move(&mut self, params: &mut MouseMoveParams) {
        if !self.active {
            let region = self.get_max_region();
            let prev_mouse = params.manager.get_mouse_position();
            let next_mouse = params.manager.to_gl_coords(params.event.get_new_position());
            if region.is_inside(prev_mouse) != region.is_inside(next_mouse) {
                params.agent.request_render();
            }
        }
    }

    fn on_click(&mut self, params: &mut MouseClickParams) {
        self.active = self.region.get_max_region().is_inside(params.manager.get_mouse_position()) && !self.active;
        params.agent.request_render();
    }
}