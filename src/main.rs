use gilrs::{
    Gilrs, GamepadId, Button, EventType, Axis
};
use enigo::*;
use std::io::{self, Write};

struct Stick {
    x: f32,
    y: f32
}

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

fn direction8 (x: f32, y: f32) -> Option<Direction> {
    let angle = x.atan2(y) * (180.0 / std::f32::consts::PI) + 180.0;
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

fn main () {
    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad = None;
    let mut input = Enigo::new();
    println!("PiWrite — Joystick input method by Santiago Cézar <santiagocezar2013@gmail.com>");

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

    loop {
        while let Some(ev) = gilrs.next_event() {
            active_gamepad = Some(ev.id);
            // println!("{:?} New event from {}: {:?}", time, id, event);
            match ev.event {
                EventType::ButtonReleased(btn, _code) => {
                    match btn {
                        Button::RightTrigger => input.key_up(Key::Backspace),
                        Button::LeftTrigger => input.key_up(Key::Space),
                        Button::LeftTrigger2 => input.key_up(Key::Shift),
                        _ => ()
                    }
                }
                EventType::ButtonPressed(btn, _code) => {
                    if let Some(letter) = match btn {
                        Button::RightTrigger => {input.key_down(Key::Backspace); None},
                        Button::LeftTrigger => {input.key_down(Key::Space); None},
                        Button::LeftTrigger2 => {input.key_down(Key::Shift); None},
                        Button::DPadUp => Some(0),
                        Button::DPadRight => Some(1),
                        Button::DPadDown => Some(2),
                        Button::DPadLeft => Some(3),
                        _ => None
                    } {
                        if let Some(gpad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                            
                            let mut left = Stick {
                                x: gpad.value(Axis::LeftStickX),
                                y: gpad.value(Axis::LeftStickY)
                            };
                            let mut right = Stick {
                                x: gpad.value(Axis::RightStickX),
                                y: gpad.value(Axis::RightStickY)
                            };
                
                            left.x *= (1.0 - 0.5 * left.y.powi(2)).sqrt();
                            left.y *= (1.0 - 0.5 * left.x.powi(2)).sqrt();
                            right.x *= (1.0 - 0.5 * right.y.powi(2)).sqrt();
                            right.y *= (1.0 - 0.5 * right.x.powi(2)).sqrt();
                
                            print!("\rLeft Hypot: {:.2}. Right Hypot: {:.2}.  ", left.x.hypot(left.y), right.x.hypot(right.y));
                            io::stdout().flush().unwrap();
                            
                            if let Some(r) = direction8(right.x, right.y) {
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
                            }
                        }
    
                    }
                },
                _ => ()
            }
        }
    }
}