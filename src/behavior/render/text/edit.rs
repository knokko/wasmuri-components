use std::cell::RefCell;
use std::rc::Rc;

use unicode_segmentation::UnicodeSegmentation;

use wasmuri_container::*;
use wasmuri_core::*;
use wasmuri_text::*;

use super::*;

pub struct EditTextRenderController {

    region: TextRegionProps,
    agent: Option<Weak<RefCell<ComponentAgent>>>,
    text_model: TextModel,

    base_colors: TextColors,
    hover_colors: TextColors,
    active_colors: TextColors,

    active: bool,
    mouse_over: bool,
    current_text: String
}

fn lighten_component(component: u8) -> u8 {
    ((component as u16 + 255) / 2) as u8
}

fn lighten_color(color: Color) -> Color {
    Color::from_rgba(lighten_component(color.get_red()), lighten_component(color.get_green()), lighten_component(color.get_blue()), lighten_component(color.get_alpha()))
}

fn to_active_colors(colors: TextColors) -> TextColors {
    TextColors::new(colors.fill_color, colors.stroke_color, lighten_color(colors.background_color))
}

fn darken_component(component: u8) -> u8 {
    ((component as u16 * 19) / 20) as u8
}

fn darken_color(color: Color) -> Color {
    Color::from_rgba(darken_component(color.get_red()), darken_component(color.get_green()), darken_component(color.get_blue()), color.get_alpha())
}

fn to_hover_colors(colors: TextColors) -> TextColors {
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
            // TODO Handle the case where mouse_over should be true initially
            mouse_over: false,
            current_text: text.to_string()
        }
    }

    pub fn celled(text: &str, font: &Rc<Font>, region: TextRegionProps, base_colors: TextColors, hover_colors: TextColors, active_colors: TextColors) -> Rc<RefCell<EditTextRenderController>> {
        Rc::new(RefCell::new(EditTextRenderController::new(text, font, region, base_colors, hover_colors, active_colors)))
    }

    pub fn simple(text: &str, font: &Rc<Font>, region: TextRegionProps, colors: TextColors) -> EditTextRenderController {
        Self::new(text, font, region, colors, to_hover_colors(colors), to_active_colors(colors))
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
        self.hover_colors = to_hover_colors(new_colors);
        self.active_colors = to_active_colors(new_colors);
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

    fn attach(&mut self, agent: &mut dyn LayerAgent){
        agent.claim_mouse_in_out_space(self.region.get_max_region());
        agent.claim_render_space(self.region.get_max_region(), RenderTrigger::Request, 
                determine_render_opacity(vec![self.base_colors, self.hover_colors, self.active_colors]), 
                RenderPhase::Text).expect("Should have render space for EditTextRenderController");
        agent.make_key_down_listener(10);
        agent.claim_mouse_click_space(self.region.get_max_region()).expect("Should have click space for EditTextRenderController");
        agent.make_copy_listener(50);
        agent.make_paste_listener(50);
        agent.make_cut_listener(50);
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.agent = Some(agent);
    }

    fn get_agent(&self) -> &Weak<RefCell<ComponentAgent>> {
        self.agent.as_ref().expect("Agent should have been set by now")
    }

    fn render(&mut self, params: &mut RenderParams) -> BehaviorRenderResult {
        let region = self.get_current_region();
        let actions = vec![PassedRenderAction::new(self.get_max_region())];
        let colors;
        let result = match self.mouse_over {
            true => BehaviorRenderResult::with_cursor(Cursor::TEXT, actions),
            false => BehaviorRenderResult::without_cursor(actions)
        };
        
        if self.active {
            colors = self.active_colors;
        } else if self.mouse_over {
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

    fn get_cursor(&mut self, _params: &mut CursorParams) -> Option<Cursor> {
        if self.mouse_over {
            Some(Cursor::TEXT)
        } else {
            None
        }
    }

    fn mouse_move(&mut self, params: &mut MouseMoveParams) {
        let region = self.get_max_region();
        let new_mouse_over = params.new_mouse_pos.is_some() && region.is_float_inside(params.new_mouse_pos.unwrap());

        if !self.active && self.mouse_over != new_mouse_over {
            self.agent().borrow_mut().request_render();
        }
        self.mouse_over = new_mouse_over;
    }

    fn mouse_click_inside(&mut self, _params: &mut MouseClickParams) {
        self.active = !self.active;
        self.agent().borrow_mut().request_render();
    }

    fn mouse_click_outside(&mut self, _params: &mut MouseClickOutParams) {
        self.active = false;
        self.agent().borrow_mut().request_render();
    }

    fn key_down(&mut self, params: &mut KeyDownParams) -> bool {
        if self.is_active() && !params.keys.is_control_down() {
            let key = params.keys.get_key();
            let mut char_counter = 0;
            {
                let mut chars = UnicodeSegmentation::graphemes(key, true);
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

    fn on_copy(&mut self) -> Option<ClipboardData> {
        match self.is_active() {
            true => match self.current_text.is_empty() {
                false => Some(ClipboardData::Text(self.current_text.clone())),
                true => None
            }, false => None
        }
    }

    fn on_paste(&mut self, clipboard: &ClipboardData) -> bool {
        match self.is_active() {
            true => match clipboard {
                ClipboardData::Text(text_to_paste) => {
                    self.current_text += text_to_paste;
                    self.update_text();
                    true
                }
            }, false => false
        }
    }

    fn on_cut(&mut self) -> Option<ClipboardData> {
        match self.is_active() {
            true => match self.current_text.is_empty() {
                false => {
                    let result = Some(ClipboardData::Text(self.current_text.clone()));
                    self.current_text = "".to_string();
                    self.update_text();
                    result
                }, true => None
            }, false => None
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