
use gilrs::{
    Gilrs, Button as gButton, EventType, Axis
};
use enigo::{
    Enigo,
    KeyboardControllable,
    Key as eKey,
};
use std::io::{self, Write};
use piston_window::*;
use glutin_window::GlutinWindow;
use ::image::{
    load_from_memory,
    DynamicImage
};

enum Direction {
    South,
    Southeast,
    East,
    Northeast,
    North,
    Northwest,
    West,
    Southwest
}

fn direction8 (x: f64, y: f64) -> Option<Direction> {
    let angle = x.atan2(y) * (180.0 / std::f64::consts::PI) + 180.0;
    let angle = (angle + 22.5) % 360.0;

    if x.hypot(y).abs() > 0.9 {
        if angle < 45.0         { Some(Direction::North) }
        else if angle < 90.0    { Some(Direction::Northwest) }
        else if angle < 135.0   { Some(Direction::West) }
        else if angle < 180.0   { Some(Direction::Southwest) }
        else if angle < 225.0   { Some(Direction::South) }
        else if angle < 270.0   { Some(Direction::Southeast) }
        else if angle < 315.0   { Some(Direction::East) }
        else if angle < 360.0   { Some(Direction::Northeast) }
        else                    { Some(Direction::North) }
    } 
    else { None }
}

struct Indicator {
    position: (f64, f64),
    radius: types::Radius
}

fn main () -> Result<(), String> {

    let mut window: PistonWindow<GlutinWindow> =
        WindowSettings::new("πWrite", [454, 454])
            .exit_on_esc(true)
            .samples(4)
            .build().expect("Failed to create window.");

    window.window.ctx.window().set_always_on_top(true);

    // Embed image in executable
    let _pie: DynamicImage = load_from_memory(include_bytes!("letterpie.png")).unwrap();
    let pie = Texture::from_image(
        &mut window.create_texture_context(),
        &_pie.to_rgba(),
        &TextureSettings::new()
    ).expect("Failed to load texture.");

    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad = None;
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
    let mut ind = Indicator {
        position: half_size,
        radius: 4.0,
    };

    let mut shift = false;
    
    let mut letter_idx = None;

    while let Some(event) = window.next() {
        
        while let Some(ev) = gilrs.next_event() {
            active_gamepad = Some(ev.id);
            // println!("{:?} New event from {}: {:?}", time, id, event);
            match ev.event {
                EventType::ButtonReleased(btn, _code) => {
                    match btn {
                        gButton::RightTrigger => input.key_up(eKey::Backspace),
                        gButton::LeftTrigger => input.key_up(eKey::Space),
                        _ => ()
                    }
                }
                EventType::ButtonPressed(btn, _code) => {
                    letter_idx = match btn {
                        gButton::RightTrigger => {input.key_down(eKey::Backspace); None},
                        gButton::LeftTrigger => {input.key_down(eKey::Space); None},
                        gButton::LeftTrigger2 => {
                            if shift {
                                input.key_up(eKey::Shift);
                                shift = false;
                            }
                            else {
                                input.key_down(eKey::Shift); 
                                shift = true;
                            }
                            None
                        },
                        gButton::DPadUp => Some(0),
                        gButton::DPadRight => Some(1),
                        gButton::DPadDown => Some(2),
                        gButton::DPadLeft => Some(3),
                        _ => None
                    }
                },
                _ => ()
            }
        }
    
        if let Some(gpad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
    
            let right = (
                (gpad.value(Axis::RightStickX)) as f64,
                (gpad.value(Axis::RightStickY)) as f64
            );
            
            let right= (
                (right.0 * (1.0 - 0.5 * right.1 * right.1).sqrt() * 100.0) as f64,
                (right.1 * (1.0 - 0.5 * right.0 * right.0).sqrt() * 100.0) as f64
            );
            
            ind.position.0 = half_size.0 + (right.0 * 1.75);
            ind.position.1 = half_size.1 - (right.1 * 1.75);
    
            io::stdout().flush().unwrap();
            
            if let Some(r) = direction8(right.0, right.1) {
                if let Some(letter) = letter_idx {
                    input.key_sequence(letter_table[match r {
                        Direction::South =>     0,
                        Direction::Southeast => 1,
                        Direction::East =>      2,
                        Direction::Northeast => 3,
                        Direction::North =>     4,
                        Direction::Northwest => 5,
                        Direction::West =>      6,
                        Direction::Southwest => 7,
                    }][letter].to_string().as_str());
                    letter_idx = None
                }
            }
        }
    
        window.draw_2d(&event, |ctx, gfx, _dev| {
            clear([1.0; 4], gfx);
            image(&pie, ctx.transform, gfx);
            ellipse([0.0,0.0,0.0,1.0],
                    ellipse::circle(ind.position.0, ind.position.1, ind.radius), 
                    ctx.transform, 
                    gfx);
        });
    }

    Ok(())
}