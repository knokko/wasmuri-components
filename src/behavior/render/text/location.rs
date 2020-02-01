use wasmuri_container::RenderParams;
use wasmuri_core::Region;

use wasmuri_text::TextModel;

pub trait TextLocationProperties {

    fn get_max_region(&self) -> Region;

    fn get_current_region(&self, text: &TextModel) -> Region;

    fn should_clear_remaining(&self, text: &TextModel, params: &mut RenderParams) -> bool;
}

pub type TextRegionProps = Box<dyn TextLocationProperties>;

#[derive(Clone,Copy)]
pub enum TextAlignment {

    LeftUp,
    LeftCenter,
    LeftDown,
    CenterUp,
    Center,
    CenterDown,
    RightUp,
    RightCenter,
    RightDown
}

pub struct AlignedTextLocation {

    max_region: Region,
    alignment: TextAlignment,
    clear_remaining: bool
}

impl AlignedTextLocation {

    pub fn new(max_region: Region, alignment: TextAlignment, clear_remaining: bool) -> AlignedTextLocation {
        AlignedTextLocation {
            max_region,
            alignment,
            clear_remaining
        }
    }
}

impl TextLocationProperties for AlignedTextLocation {

    fn get_max_region(&self) -> Region {
        self.max_region
    }

    fn get_current_region(&self, text: &TextModel) -> Region {
        let preferred_scale_y = self.max_region.get_float_height();
        let preferred_scale_x = text.get_render_width(preferred_scale_y);
        let scale_x;
        let scale_y;
        if preferred_scale_x <= self.max_region.get_float_width() {
            scale_x = preferred_scale_x;
            scale_y = preferred_scale_y;
        } else {
            scale_x = self.max_region.get_float_width();
            scale_y = preferred_scale_y * self.max_region.get_float_width() / preferred_scale_x;
        }

        let min_x = self.max_region.get_float_min_x();
        let buffer_x =self.max_region.get_float_width() - scale_x;
        let offset_x = match self.alignment {
            TextAlignment::LeftUp | TextAlignment::LeftCenter | TextAlignment::LeftDown => min_x,
            TextAlignment::CenterUp | TextAlignment::Center | TextAlignment::CenterDown => min_x + buffer_x / 2.0,
            TextAlignment::RightUp | TextAlignment::RightCenter | TextAlignment::RightDown => min_x + buffer_x
        };

        let min_y = self.max_region.get_float_min_y();
        let buffer_y = self.max_region.get_float_height() - scale_y;
        let offset_y = match self.alignment {
            TextAlignment::LeftUp | TextAlignment::CenterUp | TextAlignment::RightUp => min_y + buffer_y,
            TextAlignment::LeftCenter | TextAlignment::Center | TextAlignment::RightCenter => min_y + buffer_y / 2.0,
            TextAlignment::LeftDown | TextAlignment::CenterDown | TextAlignment::RightDown => min_y
        };

        return Region::from_floats(offset_x, offset_y, offset_x + scale_x, offset_y + scale_y);
    }

    fn should_clear_remaining(&self, _text: &TextModel, _params: &mut RenderParams) -> bool {
        self.clear_remaining
    }
}

pub fn label_location(max_region: Region, alignment: TextAlignment) -> TextRegionProps {
    Box::new(AlignedTextLocation::new(max_region, alignment, false))
}

pub fn button_location(max_region: Region) -> TextRegionProps {
    Box::new(AlignedTextLocation::new(max_region, TextAlignment::Center, false))
}

pub fn left_button_location(max_region: Region) -> TextRegionProps {
    Box::new(AlignedTextLocation::new(max_region, TextAlignment::LeftCenter, false))
}

pub fn edit_location(max_region: Region) -> TextRegionProps {
    Box::new(AlignedTextLocation::new(max_region, TextAlignment::LeftCenter, true))
}