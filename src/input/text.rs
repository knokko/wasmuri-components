use crate::helper::render::text::*;

use std::rc::Rc;

use unicode_segmentation::UnicodeSegmentation;

use wasmuri_container::{
    Cursor,
    Component,
    layer::LayerAgent,
    params::*
};

use wasmuri_text::Font;

pub struct TextEditField {

    render_helper: Box<EditTextRenderHelper>,
    font: Rc<Font>,

    current_text: String
}

impl TextEditField {

    pub fn new(start_text: String, font: &Rc<Font>, render_helper: Box<EditTextRenderHelper>) -> TextEditField {
        TextEditField {
            render_helper,
            font: Rc::clone(font),
            current_text: start_text
        }
    }
}

impl Component for TextEditField {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.attach(agent).expect("Should have space for the text render helper of this edit field");
        agent.make_key_down_listener(10);
        agent.make_mouse_click_listener();
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor>{
        self.render_helper.render(params)
    }

    fn get_cursor(&mut self, params: &mut CursorParams) -> Option<Cursor> {
        self.render_helper.get_cursor(params)
    }

    fn mouse_move(&mut self, params: &mut MouseMoveParams) {
        self.render_helper.on_mouse_move(params);
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams){
        self.render_helper.on_click(params);
    }

    fn key_down(&mut self, params: &mut KeyDownParams) -> bool {
        if self.render_helper.is_active() {
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
                    self.render_helper.set_active(false);
                }
            }

            self.render_helper.set_text(&self.current_text, Rc::clone(&self.font), params.agent);
            true
        } else {
            false
        }
    }
}