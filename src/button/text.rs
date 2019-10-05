use crate::helper::render::text::TextRenderHelper;

use wasmuri_container::Component;
use wasmuri_container::Cursor;
use wasmuri_container::layer::LayerAgent;
use wasmuri_container::params::*;

pub struct TextButton {

    render_helper: Box<dyn TextRenderHelper>,
    on_click: Box<dyn FnMut(&mut dyn TextRenderHelper, &mut MouseClickParams)>
}

impl TextButton {

    pub fn new(render_helper: Box<dyn TextRenderHelper>, on_click: Box<dyn FnMut(&mut dyn TextRenderHelper, &mut MouseClickParams)>) -> TextButton {
        TextButton {
            render_helper,
            on_click
        }
    }

    pub fn boxed(render_helper: Box<dyn TextRenderHelper>, on_click: Box<dyn FnMut(&mut dyn TextRenderHelper, &mut MouseClickParams)>) -> Box<TextButton> {
        Box::new(TextButton::new(render_helper, on_click))
    }
}

impl Component for TextButton {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.attach(agent).expect("Should have render space for TextButton");
        agent.claim_mouse_click_space(self.render_helper.get_max_region()).expect("Should have click space for TextButton");
    }

    fn mouse_click(&mut self, params: &mut MouseClickParams){
        self.on_click.as_mut()(self.render_helper.as_mut(), params);
    }

    fn mouse_move(&mut self, params: &mut MouseMoveParams){
        self.render_helper.on_mouse_move(params);
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor> {
        self.render_helper.render(params)
    }

    fn get_cursor(&mut self, params: &mut CursorParams) -> Option<Cursor> {
        self.render_helper.get_cursor(params)
    }
}