use std::cell::RefCell;
use std::rc::Rc;

use unicode_segmentation::UnicodeSegmentation;

use wasmuri_container::Cursor;
use wasmuri_container::layer::*;
use wasmuri_container::params::*;

use wasmuri_core::color::*;

use wasmuri_text::{
    Font,
    TextModel
};

use super::*;

pub struct EditTextRenderController {

    region: TextRegionProps,
    agent: Option<Weak<RefCell<ComponentAgent>>>,
    text_model: TextModel,

    base_colors: TextColors,
    hover_colors: TextColors,
    active_colors: TextColors,

    active: bool,
    current_text: String
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

impl EditTextRenderController {

    pub fn new(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors, active_colors: TextColors) -> EditTextRenderController {
        EditTextRenderController {
            region,
            agent: None,
            text_model: Rc::clone(font).create_text_model(text),

            base_colors,
            hover_colors,
            active_colors,

            active: false,
            current_text: text.to_string()
        }
    }

    pub fn celled(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors, active_colors: TextColors) -> Rc<RefCell<EditTextRenderController>> {
        Rc::new(RefCell::new(EditTextRenderController::new(text, font, region, base_colors, hover_colors, active_colors)))
    }

    pub fn simple(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> EditTextRenderController {
        Self::new(text, font, region, colors, darken_colors(colors), lighten_colors(colors))
    }

    pub fn simple_celled(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> Rc<RefCell<EditTextRenderController>> {
        Rc::new(RefCell::new(Self::simple(text, font, region, colors)))
    }

    pub fn tuple(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors, active_colors: TextColors) -> (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<EditTextRenderController>>) {
        let instance = Rc::new(RefCell::new(Self::new(text, font, region, base_colors, hover_colors, active_colors)));
        (Rc::clone(&instance) as Rc<RefCell<dyn ComponentBehavior>>, instance)
    }

    pub fn simple_tuple(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> (Rc<RefCell<dyn ComponentBehavior>>, Rc<RefCell<EditTextRenderController>>) {
        let instance = Rc::new(RefCell::new(Self::simple(text, font, region, colors)));
        (Rc::clone(&instance) as Rc<RefCell<dyn ComponentBehavior>>, instance)
    }

    // TODO Update render opacity if necessary!

    pub fn set_base_fill_color(&mut self, new_color: Color){
        self.base_colors.fill_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_base_stroke_color(&mut self, new_color: Color){
        self.base_colors.stroke_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_base_background_color(&mut self, new_color: Color){
        self.base_colors.background_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_base_colors(&mut self, new_colors: TextColors){
        self.base_colors = new_colors;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_hover_fill_color(&mut self, new_color: Color){
        self.hover_colors.fill_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_hover_stroke_color(&mut self, new_color: Color){
        self.hover_colors.stroke_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_hover_background_color(&mut self, new_color: Color){
        self.hover_colors.background_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_hover_colors(&mut self, new_colors: TextColors){
        self.hover_colors = new_colors;
        self.agent().borrow_mut().request_render();    
    }

    pub fn set_active_fill_color(&mut self, new_color: Color){
        self.active_colors.fill_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_active_stroke_color(&mut self, new_color: Color){
        self.active_colors.stroke_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_active_background_color(&mut self, new_color: Color){
        self.active_colors.background_color = new_color;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_active_colors(&mut self, new_colors: TextColors){
        self.active_colors = new_colors;
        self.agent().borrow_mut().request_render();
    }

    pub fn set_fill_color(&mut self, new_color: Color){
        self.base_colors.fill_color = new_color;
        self.hover_colors.fill_color = darken_color(new_color);
        self.active_colors.fill_color = lighten_color(new_color);
        self.agent().borrow_mut().request_render();
    }

    pub fn set_stroke_color(&mut self, new_color: Color){
        self.base_colors.stroke_color = new_color;
        self.hover_colors.stroke_color = darken_color(new_color);
        self.active_colors.stroke_color = lighten_color(new_color);
        self.agent().borrow_mut().request_render();
    }

    pub fn set_background_color(&mut self, new_color: Color){
        self.base_colors.background_color = new_color;
        self.hover_colors.background_color = darken_color(new_color);
        self.active_colors.background_color = lighten_color(new_color);
        self.agent().borrow_mut().request_render();
    }

    pub fn set_colors(&mut self, new_colors: TextColors){
        self.base_colors = new_colors;
        self.hover_colors = darken_colors(new_colors);
        self.active_colors = lighten_colors(new_colors);
        self.agent().borrow_mut().request_render();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, new_active: bool) {
        self.active = new_active;
    }

    pub fn get_current_text(&self) -> &str {
        &self.current_text
    }

    fn agent(&self) -> Rc<RefCell<ComponentAgent>> {
        self.agent.as_ref().expect("Component agent should have been set by now").upgrade().expect("Component agent should not have been dropped")
    }

    fn update_text(&mut self){
        self.text_model = Rc::clone(self.text_model.get_font()).create_text_model(&self.current_text);
        self.agent().borrow_mut().request_render();
    }
}

impl ComponentBehavior for EditTextRenderController {

    fn attach(&mut self, agent: &mut LayerAgent){
        agent.claim_mouse_in_out_space(self.region.get_max_region());
        agent.claim_render_space(self.region.get_max_region(), RenderTrigger::Request, 
                determine_render_opacity(vec![self.base_colors, self.hover_colors, self.active_colors]), 
                RenderPhase::Text).expect("Should have render space for EditTextRenderController");
        agent.make_key_down_listener(10);
        agent.make_mouse_click_listener();
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.agent = Some(agent);
    }

    fn get_agent(&self) -> &Weak<RefCell<ComponentAgent>> {
        self.agent.as_ref().expect("Agent should have been set by now")
    }

    fn render(&mut self, params: &mut RenderParams) -> BehaviorRenderResult {
        let region = self.get_max_region();
        let colors;
        let result = match region.is_float_inside(params.manager.get_mouse_position()) {
            true => BehaviorRenderResult::with_cursor(Cursor::TEXT),
            false => BehaviorRenderResult::without_cursor()
        };
        
        if self.active {
            colors = self.active_colors;
        } else if result.has_cursor() {
            colors = self.hover_colors;
        } else {
            colors = self.base_colors;
        }

        if self.region.should_clear_remaining(&self.text_model, params) {
            self.text_model.get_font().fill_rect(self.region.get_max_region(), colors.background_color);
        }

        let render_width = self.text_model.get_render_width(region.get_float_height());
        let render_height;
        if render_width <= region.get_float_width() {
            render_height = region.get_float_height();
        } else {
            render_height = region.get_float_height() * region.get_float_width() / render_width;
        }

        self.text_model.render(region.get_float_min_x(), region.get_float_min_y(), render_height, colors);
        result
    }

    fn get_cursor(&mut self, params: &mut CursorParams) -> Option<Cursor> {
        let region = self.get_max_region();
        if region.is_float_inside(params.manager.get_mouse_position()) {
            Some(Cursor::TEXT)
        } else {
            None
        }
    }

    fn mouse_move(&mut self, params: &mut MouseMoveParams) {
        if !self.active {
            let region = self.get_max_region();
            let prev_mouse = params.manager.get_mouse_position();
            let next_mouse = params.manager.to_gl_coords(params.event.get_new_position());
            if region.is_float_inside(prev_mouse) != region.is_float_inside(next_mouse) {
                self.agent().borrow_mut().request_render();
            }
        }
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams) {
        self.active = self.region.get_max_region().is_float_inside(params.manager.get_mouse_position()) && !self.active;
        self.agent().borrow_mut().request_render();
    }

    fn key_down(&mut self, params: &mut KeyDownParams) -> bool {
        if self.is_active() {
            let key = params.event.key_event.key();
            let mut char_counter = 0;
            {
                let mut chars = UnicodeSegmentation::graphemes(key.as_str(), true);
                let mut next_char = chars.next();
                while next_char.is_some() {
                    char_counter += 1;
                    next_char = chars.next();
                }
            }
            if char_counter < 3 {
                self.current_text += &key;
            } else {
                if key == "Backspace" {
                    let mut chars = UnicodeSegmentation::graphemes(self.current_text.as_str(), true);
                    let mut new_text = "".to_string();
                    let mut maybe_current_char = chars.next();
                    while maybe_current_char.is_some() {
                        let next_char = chars.next();
                        let current_char = maybe_current_char.unwrap();
                        if next_char.is_some() {
                            new_text += current_char;
                        }
                        maybe_current_char = next_char;
                    }
                    self.current_text = new_text;
                } else if key == "Escape" || key == "Enter" {
                    self.set_active(false);
                }
            }

            self.update_text();
            true
        } else {
            false
        }
    }
}

impl TextRenderController for EditTextRenderController {

    fn get_max_region(&self) -> Region {
        self.region.get_max_region()
    }

    fn get_current_region(&self) -> Region {
        self.region.get_current_region(&self.text_model)
    }

    fn set_text(&mut self, new_text: &str){
        self.current_text = new_text.to_string();
        self.update_text();
    }

    fn set_text_model(&mut self, new_text: TextModel){
        self.text_model = new_text;
        self.agent().borrow_mut().request_render();
    }
}