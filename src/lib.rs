use gilrs::{
    Gilrs, GamepadId, Button, EventType, Axis
};
use enigo::*;
use std::io::{self, Write};
use gdnative::*;

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
#[derive(NativeClass)]
#[inherit(Node)]
pub struct PiWrite {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
    input: Enigo,
}

#[allow(dead_code)]
#[methods]
impl PiWrite {
    
    fn _init (_parent: Node) -> PiWrite {
        PiWrite {
            gilrs: Gilrs::new().unwrap(),
            active_gamepad: None,
            input: Enigo::new()
        }
    }

    #[export]
    fn _ready(&self, _parent: Node) {
        godot_print!("PiWrite — Joystick input method by Santiago Cézar <santiagocezar2013@gmail.com>");
    }
    #[export]
    fn _process(&mut self, _parent: Node, _delta: f64) {

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

        while let Some(ev) = self.gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);
            match ev.event {
                EventType::ButtonReleased(btn, _code) => {
                    match btn {
                        Button::RightTrigger => self.input.key_up(Key::Backspace),
                        Button::LeftTrigger => self.input.key_up(Key::Space),
                        Button::LeftTrigger2 => self.input.key_up(Key::Shift),
                        _ => ()
                    }
                }
                EventType::ButtonPressed(btn, _code) => {
                    if let Some(letter) = match btn {
                        Button::RightTrigger => {self.input.key_down(Key::Backspace); None},
                        Button::LeftTrigger => {self.input.key_down(Key::Space); None},
                        Button::LeftTrigger2 => {self.input.key_down(Key::Shift); None},
                        Button::DPadUp => Some(0),
                        Button::DPadRight => Some(1),
                        Button::DPadDown => Some(2),
                        Button::DPadLeft => Some(3),
                        _ => None
                    } {
                        if let Some(gpad) = self.active_gamepad.map(|id| self.gilrs.gamepad(id)) {
                            
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
                                self.input.key_sequence(letter_table[match r {
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
            self.active_gamepad = Some(ev.id);
        }
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<PiWrite>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();