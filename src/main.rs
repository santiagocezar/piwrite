
use gilrs::Button as gButton;
use enigo::{
    Enigo,
    KeyboardControllable,
    Key as eKey,
};
use piston_window::*;
use winit::Icon;
use ::image::{
    load_from_memory,
    DynamicImage
};

mod pie;
use pie::{
    Pie, Action, Direction
};

fn main () -> Result<(), String> {

    let mut window: PistonWindow =
        WindowSettings::new("πWrite", [454, 454])
            .exit_on_esc(true)
            .samples(4)
            .build().expect("Failed to create window.");

    window.window.ctx.window().set_always_on_top(true);

    let icon = load_from_memory(include_bytes!("icon.png")).unwrap().to_rgba();
    window.window.ctx.window().set_window_icon(Some(Icon::from_rgba(
        icon.into_raw(),
        256,
        256
    ).expect("Failed to load icon")));

    // Embed image in executable
    let _pie: DynamicImage = load_from_memory(include_bytes!("letterpie.png")).unwrap();
    let pie = Texture::from_image(
        &mut window.create_texture_context(),
        &_pie.to_rgba(),
        &TextureSettings::new()
    ).expect("Failed to load texture.");

    let mut input = Enigo::new();

    let letter_table = [
        ['a', 'b', 'c', 'd'],
        ['e', 'f', 'g', 'h'],
        ['i', 'j', 'k', 'l'],
        ['m', 'n', 'ñ', 'o'],
        ['p', 'q', 'r', 's'],
        ['t', 'u', 'v', 'w'],
        ['x', 'y', 'z', ' '],
        ['.', ',', '-', '<'],
    ];

    let size = window.size();
    let half_size = (size.width / 2.0, size.height / 2.0);

    let mut shift = false;
    
    let mut letter_pie = Pie::new(8, 0.9);

    while let Some(event) = window.next() {
        
        while let Some(p) = letter_pie.update() {
            let mut l = None; 
            match p {
                Action::Press(dir) => match dir {
                    Direction::South(slice) => {
                        l = Some(letter_table[slice as usize][0])
                    },
                    Direction::East(slice) => {
                        l = Some(letter_table[slice as usize][1])
                    },
                    Direction::North(slice) => {
                        l = Some(letter_table[slice as usize][2])
                    },
                    Direction::West(slice) => {
                        l = Some(letter_table[slice as usize][3])
                    },
                    Direction::Other(btn) => {
                        match btn {
                            gButton::RightTrigger => { input.key_down(eKey::Backspace) },
                            gButton::LeftTrigger => { input.key_down(eKey::Space) },
                            gButton::LeftTrigger2 => {
                                if shift {
                                    input.key_up(eKey::Shift);
                                    shift = false;
                                }
                                else {
                                    input.key_down(eKey::Shift); 
                                    shift = true;
                                }
                            },
                            _ => ()
                        }
                    }
                },

                Action::Release(dir) => {
                    l = None;
                    if let Direction::Other(btn) = dir {
                        match btn {
                            gButton::RightTrigger => { input.key_up(eKey::Backspace) },
                            gButton::LeftTrigger => { input.key_up(eKey::Space) },
                            _ => ()
                        }
                    }
                }
            }
            if let Some(k) = l {
                input.key_click(eKey::Layout(k));
            }
        }
    
        window.draw_2d(&event, |ctx, gfx, _dev| {
            clear([1.0; 4], gfx);
            image(&pie, ctx.transform, gfx);
            let x = half_size.0 + letter_pie.x * (1.0 - 0.5 * letter_pie.y * letter_pie.y).sqrt() * 175.0;
            let y = half_size.1 + letter_pie.y * (1.0 - 0.5 * letter_pie.x * letter_pie.x).sqrt() * 175.0;
            ellipse([0.0,0.0,0.0,1.0],
                    ellipse::circle(x, y, 4.0), 
                    ctx.transform, 
                    gfx);
        });
    }

    Ok(())
}
