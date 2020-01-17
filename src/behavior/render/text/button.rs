use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_container::*;
use wasmuri_core::color::*;
use wasmuri_text::*;

use super::*;

pub struct ButtonTextRenderController {

    region: TextRegionProps,
    agent: Option<Weak<RefCell<ComponentAgent>>>,
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

impl ButtonTextRenderController {

    pub fn new(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors) -> ButtonTextRenderController {
        ButtonTextRenderController {
            region,
            agent: None,
            text_model: Rc::clone(font).create_text_model(text),

            base_colors,
            hover_colors
        }
    }

    pub fn simple(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> ButtonTextRenderController {
        Self::new(text, font, region, colors, lighten_colors(colors))
    }

    pub fn celled(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors) -> Rc<RefCell<ButtonTextRenderController>> {
        Rc::new(RefCell::new(ButtonTextRenderController::new(text, font, region, base_colors, hover_colors)))
    }

    pub fn simple_celled(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> Rc<RefCell<ButtonTextRenderController>> {
        Rc::new(RefCell::new(Self::simple(text, font, region, colors)))
    }

    pub fn tuple(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors) -> (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<dyn TextRenderController>>) {
        let instance = Rc::new(RefCell::new(Self::new(text, font, region, base_colors, hover_colors)));
        (Rc::clone(&instance) as Rc<RefCell<dyn ComponentBehavior>>, instance)
    }

    // TODO Update render opacity if necessary!

    pub fn simple_tuple(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<dyn TextRenderController>>) {
        let instance = Rc::new(RefCell::new(Self::simple(text, font, region, colors)));
        (Rc::clone(&instance) as Rc<RefCell<dyn ComponentBehavior>>, instance)
    }

    pub fn set_base_fill_color(&mut self, new_color: Color){
        self.base_colors.fill_color = new_color;
        self.request_render();
    }

    pub fn set_base_stroke_color(&mut self, new_color: Color){
        self.base_colors.stroke_color = new_color;
        self.request_render();
    }

    pub fn set_base_background_color(&mut self, new_color: Color){
        self.base_colors.background_color = new_color;
        self.request_render();
    }

    pub fn set_base_colors(&mut self, new_colors: TextColors){
        self.base_colors = new_colors;
        self.request_render();
    }

    pub fn set_hover_fill_color(&mut self, new_color: Color){
        self.hover_colors.fill_color = new_color;
        self.request_render();
    }

    pub fn set_hover_stroke_color(&mut self, new_color: Color){
        self.hover_colors.stroke_color = new_color;
        self.request_render();
    }

    pub fn set_hover_background_color(&mut self, new_color: Color){
        self.hover_colors.background_color = new_color;
        self.request_render();
    }

    pub fn set_hover_colors(&mut self, new_colors: TextColors){
        self.hover_colors = new_colors;
        self.request_render();
    }

    pub fn set_fill_color(&mut self, new_color: Color){
        self.base_colors.fill_color = new_color;
        self.hover_colors.fill_color = lighten_color(new_color);
        self.request_render();
    }

    pub fn set_stroke_color(&mut self, new_color: Color){
        self.base_colors.stroke_color = new_color;
        self.hover_colors.stroke_color = lighten_color(new_color);
        self.request_render();
    }

    pub fn set_background_color(&mut self, new_color: Color){
        self.base_colors.background_color = new_color;
        self.hover_colors.background_color = lighten_color(new_color);
        self.request_render();
    }

    pub fn set_colors(&mut self, new_colors: TextColors){
        self.base_colors = new_colors;
        self.hover_colors = lighten_colors(new_colors);
        self.request_render();
    }

    fn request_render(&self){
        self.agent.as_ref().expect("Agent should have been set by now").upgrade().expect("Agent should not have been dropped").borrow_mut().request_render();
    }
}

impl ComponentBehavior for ButtonTextRenderController {

    fn attach(&mut self, agent: &mut LayerAgent){
        agent.claim_mouse_move_space(self.region.get_max_region());
        agent.claim_render_space(self.region.get_max_region(), RenderTrigger::Request, 
                determine_render_opacity(vec![self.base_colors, self.hover_colors]), 
                RenderPhase::Text).expect("Should have render space for ButtonTextRenderController");
    }

    fn get_agent(&self) -> &Weak<RefCell<ComponentAgent>> {
        self.agent.as_ref().expect("Agent should have been set by now")
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.agent = Some(agent);
    }

    fn render(&mut self, params: &mut RenderParams) -> BehaviorRenderResult {
        let region = self.get_current_region();
        let colors;
        let result;
        
        if region.is_float_inside(params.manager.get_mouse_position()) {
            colors = self.hover_colors;
            result = BehaviorRenderResult::with_cursor(Cursor::POINTER);
        } else {
            colors = self.base_colors;
            result = BehaviorRenderResult::without_cursor();
        }

        if self.region.should_clear_remaining(&self.text_model, params) {
            self.text_model.get_font().fill_rect(self.region.get_max_region(), colors.background_color);
        }

        self.text_model.render(region.get_float_min_x(), region.get_float_min_y(), region.get_float_height(), colors);
        result
    }

    fn get_cursor(&mut self, params: &mut CursorParams) -> Option<Cursor> {
        let region = self.get_current_region();
        if region.is_float_inside(params.manager.get_mouse_position()) {
            Some(Cursor::POINTER)
        } else {
            None
        }
    }

    fn mouse_move(&mut self, params: &mut MouseMoveParams) {
        let region = self.get_current_region();
        let prev_mouse = params.manager.get_mouse_position();
        let next_mouse = params.manager.to_gl_coords(params.event.get_new_position());
        if region.is_float_inside(prev_mouse) != region.is_float_inside(next_mouse) {
            self.request_render();
        }
    }
}

impl TextRenderController for ButtonTextRenderController {

    fn get_max_region(&self) -> Region {
        self.region.get_max_region()
    }

    fn get_current_region(&self) -> Region {
        self.region.get_current_region(&self.text_model)
    }

    fn set_text(&mut self, new_text: &str){
        self.text_model = Rc::clone(self.text_model.get_font()).create_text_model(new_text);
        self.request_render();
    }

    fn set_text_model(&mut self, new_text: TextModel){
        self.text_model = new_text;
        self.request_render();
    }
}