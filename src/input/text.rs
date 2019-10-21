use crate::helper::render::text::*;

use std::cell::RefCell;
use std::rc::*;

use unicode_segmentation::UnicodeSegmentation;

use wasmuri_container::{
    Cursor,
    Component,
    layer::*,
    params::*
};

pub struct TextEditField {

    render_helper: Rc<RefCell<EditTextRenderHelper>>,

    current_text: String
}

impl TextEditField {

    pub fn new(start_text: String, render_helper: Rc<RefCell<EditTextRenderHelper>>) -> TextEditField {
        TextEditField {
            render_helper,
            current_text: start_text
        }
    }

    pub fn get_current_text(&self) -> &str {
        &self.current_text
    }
}

impl Component for TextEditField {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.borrow_mut().attach(agent).expect("Should have space for the text render helper of this edit field");
        agent.make_key_down_listener(10);
        agent.make_mouse_click_listener();
    }

    fn set_agent(&mut self, agent: Weak<RefCell<ComponentAgent>>){
        self.render_helper.borrow_mut().set_agent(agent);
    }

    fn get_agent(&self) -> Weak<RefCell<ComponentAgent>> {
        Weak::clone(self.render_helper.borrow().get_agent())
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor>{
        self.render_helper.borrow().render(params)
    }

    fn get_cursor(&mut self, params: &mut CursorParams) -> Option<Cursor> {
        self.render_helper.borrow().get_cursor(params)
    }

    fn mouse_move(&mut self, params: &mut MouseMoveParams) {
        self.render_helper.borrow_mut().on_mouse_move(params);
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams){
        self.render_helper.borrow_mut().on_click(params);
    }

    fn key_down(&mut self, params: &mut KeyDownParams) -> bool {
        if self.render_helper.borrow().is_active() {
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
                    self.render_helper.borrow_mut().set_active(false);
                }
            }

            self.render_helper.borrow_mut().set_text(&self.current_text);
            true
        } else {
            false
        }
    }
}