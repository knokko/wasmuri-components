mod simple;
mod button;
mod edit;
mod location;

pub use simple::*;
pub use button::*;
pub use edit::*;
pub use location::*;

use std::rc::*;

use wasmuri_container::*;
use wasmuri_core::util::Region;
use wasmuri_core::color::*;
use wasmuri_text::TextModel;

pub trait TextRenderController {

    fn set_text_model(&mut self, new_text: TextModel);

    fn set_text(&mut self, new_text: &str);

    fn get_max_region(&self) -> Region;

    fn get_current_region(&self) -> Region;
}

fn determine_render_opacity(colors: Vec<TextColors>) -> RenderOpacity {
    
    let mut all_solid = true;
    for color in &colors {
        if !color.is_fully_solid() {
            all_solid = false;
            break;
        }
    }

    if all_solid {
        return RenderOpacity::StaticSolidOrNothing;
    }

    let mut has_partial_transparancy = false;
    for color in &colors {
        if color.has_partial_transparency() {
            has_partial_transparancy = true;
            break;
        }
    }

    if has_partial_transparancy {
        RenderOpacity::Mixed
    } else {
        RenderOpacity::DynamicSolidOrNothing
    }
}