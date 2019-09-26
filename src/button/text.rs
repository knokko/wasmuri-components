use crate::helper::render::text::TextRenderHelper;

use wasmuri_container::{
    Component,
    ContainerManager
};
use wasmuri_container::cursor::Cursor;
use wasmuri_container::layer::{
    ComponentAgent,
    LayerAgent
};
use wasmuri_events::{
    MouseClickEvent,
    RenderEvent
};

use web_sys::WebGlRenderingContext;

pub struct TextButton {

    render_helper: Box<dyn TextRenderHelper>,
    on_click: Box<dyn FnMut(&mut dyn TextRenderHelper, &mut ComponentAgent, &MouseClickEvent, &ContainerManager)>
}

impl TextButton {

    pub fn new(render_helper: Box<dyn TextRenderHelper>, on_click: Box<dyn FnMut(&mut dyn TextRenderHelper, &mut ComponentAgent, &MouseClickEvent, &ContainerManager)>) -> TextButton {
        TextButton {
            render_helper,
            on_click
        }
    }
}

impl Component for TextButton {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.attach(agent).expect("Should have render space for TextButton");
        agent.claim_mouse_click_space(self.render_helper.get_max_region()).expect("Should have click space for TextButton");
    }

    fn mouse_click(&mut self, agent: &mut ComponentAgent, event: &MouseClickEvent, manager: &ContainerManager){
        self.on_click.as_mut()(self.render_helper.as_mut(), agent, event, manager);
    }

    fn render(&mut self, _gl: &WebGlRenderingContext, _agent: &mut ComponentAgent, _event: &RenderEvent, _manager: &ContainerManager) -> Option<Cursor> {
        self.render_helper.render();
        Some(Cursor::POINTER)
    }

    fn get_cursor(&mut self, _agent: &mut ComponentAgent, _event: &RenderEvent, _manager: &ContainerManager) -> Option<Cursor> {
        Some(Cursor::POINTER)
    }
}