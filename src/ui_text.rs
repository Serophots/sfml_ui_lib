use sfml::graphics::{Color, Drawable, Font, RenderStates, RenderTarget, Text, Transformable};
use crate::udim::{UDim2, UIPositionable, UISizeable};

pub struct UIText<'s> {
    text: Text<'s>,
    position: UDim2,
}
impl<'s> UIText<'s> {
    pub fn new(text: String, font: &Font, char_size: u32, position: UDim2, color: Color, position_centrally: bool) -> Self {
        let mut text = Text::new(text, font, char_size);

        text.set_fill_color(color);
        text.set_outline_color(Color::BLACK);
        text.set_outline_thickness(3.);

        if position_centrally {
            text.set_position(position.get_absolute());
        } else {
            text.set_position((position.get_absolute().x - (text.global_bounds().size().x / 2.), position.get_absolute().y));

        }

        Self {
            text,
            position
        }
    }
}

impl<'s> Drawable for UIText<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw_with_renderstates(&self.text, states);
    }
}

impl<'s> UIPositionable for UIText<'s> {
    fn get_position(&self) -> UDim2 {
        self.position
    }

    fn set_position(&mut self, position: UDim2) {
        self.position = position;
    }
}

impl<'s> UISizeable for UIText<'s> {
    fn get_size(&self) -> UDim2 {
        self.size
    }

    fn set_size(&mut self, size: UDim2) {
        self.size = size;
    }
}