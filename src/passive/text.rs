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
use wasmuri_events::RenderEvent;

use web_sys::WebGlRenderingContext;

pub struct PassiveText {

    render_helper: Box<dyn TextRenderHelper>
}

impl Component for PassiveText {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.attach(agent).expect("Space should be free");
    }

    fn render(&mut self, _gl: &WebGlRenderingContext, _agent: &mut ComponentAgent, _event: &RenderEvent, _manager: &ContainerManager) -> Option<Cursor> {
        self.render_helper.render();
        None
    }
}