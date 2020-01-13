use std::cell::RefCell;
use std::rc::Rc;

use crate::behavior::render::*;
use crate::button::text::TextButton;
use crate::input::text::TextEditField;
use wasmuri_container::Component;
use wasmuri_container::layer::*;
use wasmuri_core::color::*;
use wasmuri_core::util::Region;
use wasmuri_container::params::MouseClickParams;
use wasmuri_text::Font;

pub fn add_simple_text_button<C: FnMut(&mut ComponentAgent, &mut dyn TextRenderController, &mut MouseClickParams) + 'static>
        (layer: &mut Layer, min_x: i32, min_y: i32, max_x: i32, max_y: i32, 
        text: &str, button_color: Color, font: &Rc<Font>, alignment: TextAlignment, on_click: C) {

    layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple(text, font,
            Box::new(AlignedTextLocation::new(Region::new(min_x, min_y, max_x, max_y), alignment, false)),
            TextColors::create_simple_button(button_color)
    ), Box::new(on_click)));
}

pub fn add_simple_edit_field(layer: &mut Layer, min_x: i32, min_y: i32, max_x: i32, max_y: i32, initial_text: &str, font: &Rc<Font>) 
        -> Rc<RefCell<TextEditField>> {

    let field_cell = TextEditField::celled(EditTextRenderController::simple_tuple(initial_text, &font, 
        Box::new(AlignedTextLocation::new(Region::new(min_x, min_y, max_x, max_y), TextAlignment::LeftCenter, true)), 
        TextColors::new(Color::BLACK, Color::BLACK, Color::from_rgb(150, 150, 150))));

    layer.add_component(Rc::clone(&field_cell) as Rc<RefCell<dyn Component>>);

    field_cell
}