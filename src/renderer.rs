use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use specs::{Join, ReadStorage};

use crate::components::*;

pub type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            current_frame.width(),
            current_frame.height(),
        );
        canvas.copy(&texture[sprite.spritesheet], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}
