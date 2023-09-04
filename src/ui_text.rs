use sfml::graphics::{Color, Drawable, Font, RenderStates, RenderTarget, Text, Transformable};
use crate::udim::{UDim2, UIPositionable, UISizeable};

pub struct UIText<'s> {
    text: Text<'s>,
    position: UDim2,
}
impl<'s> UIText<'s> {
    pub fn new(text: &str, font: &'s Font, char_size: u32, position: UDim2, color: Color, position_centered_x: bool, position_centered_y: bool) -> Self {
        let mut text = Text::new(text, font, char_size);

        text.set_fill_color(color);
        text.set_outline_color(Color::BLACK);
        text.set_outline_thickness(3.);

        let mut x = position.get_absolute().x;
        let mut y = position.get_absolute().y;

        if position_centered_x {
            x = position.get_absolute().x - (text.global_bounds().size().x / 2.)
        }
        if position_centered_y {
            y = position.get_absolute().y - (char_size as f32 /2.)
        }
        text.set_position((x,y));

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
        UDim2::from_absolute((self.text.global_bounds().size().x, self.text.global_bounds().size().y))
    }

    fn set_size(&mut self, size: UDim2) {
        unreachable!()
    }
}