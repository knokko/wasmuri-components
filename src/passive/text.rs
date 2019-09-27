use crate::helper::render::text::TextRenderHelper;

use wasmuri_container::Component;
use wasmuri_container::Cursor;
use wasmuri_container::layer::LayerAgent;
use wasmuri_container::params::RenderParams;

pub struct PassiveText {

    render_helper: Box<dyn TextRenderHelper>
}

impl Component for PassiveText {

    fn attach(&mut self, agent: &mut LayerAgent){
        self.render_helper.attach(agent).expect("Space should be free");
    }

    fn render(&mut self, params: &mut RenderParams) -> Option<Cursor> {
        self.render_helper.render(params);
        None
    }
}