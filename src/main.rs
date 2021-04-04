mod animator;
mod components;
mod physics;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use specs::{Builder, World, WorldExt};

use std::time::Duration;

use crate::components::*;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

fn get_spritesheet_row_from_direction(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

fn create_character_animation_frames(
    spritesheet: usize,
    initial_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = initial_frame.size();
    let y_offset =
        initial_frame.y() + frame_height as i32 * get_spritesheet_row_from_direction(direction);

    let mut frames = Vec::new();

    for index in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                initial_frame.x() + frame_width as i32 * index,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y()
            + frame_height as i32 * get_spritesheet_row_from_direction(player.direction),
        frame_width,
        frame_height,
    );

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Three Lines", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize game window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let textures = [texture_creator.load_texture("assets/bardo.png")?];

    let player_spritesheet = 0;
    let player_initial_frame = Rect::new(0, 0, 26, 36);
    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: create_character_animation_frames(
            player_spritesheet,
            player_initial_frame,
            Direction::Up,
        ),
        down_frames: create_character_animation_frames(
            player_spritesheet,
            player_initial_frame,
            Direction::Down,
        ),
        left_frames: create_character_animation_frames(
            player_spritesheet,
            player_initial_frame,
            Direction::Left,
        ),
        right_frames: create_character_animation_frames(
            player_spritesheet,
            player_initial_frame,
            Direction::Right,
        ),
    };

    let mut world = World::new();

    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.speed = 0;
                }
                _ => {}
            }
        }

        // Later we'll be used for update()
        i = (i + 1) % 255;

        render(&mut canvas, Color::RGB(i, 125, 255 - i), &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 24));
    }

    Ok(())
}
