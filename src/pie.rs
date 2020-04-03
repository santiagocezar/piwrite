use gilrs::{
    Gilrs, EventType, Axis
};

pub use gilrs::Button as piButton;

pub struct Pie {
    pub slices: u8,
    pub deadzone: f64,
    gilrs: Gilrs,
    pub x: f64,
    pub y: f64
}
/// Direction of the D-Pad containing the selected slice 
pub enum Action {
    Press(Direction),
    Release(Direction)
}

pub enum Direction {
    South(u8),
    North(u8),
    East(u8),
    West(u8),
    Other(piButton)
}

impl Pie {
    pub fn new(slices: u8, deadzone: f64) -> Self {
        Self {
            slices,
            deadzone,
            gilrs: Gilrs::new().expect("Failed to create Gilrs."),
            x: 0.0, y: 0.0
        }
    }

    fn get_slice(&self) -> Option<u8> {
        let slice_angle = 360.0 / self.slices as f64;
        let angle = (-self.x).atan2(self.y) * (180.0 / std::f64::consts::PI) + 180.0;
        let angle = (angle + slice_angle / 2.0) % 360.0;
    
        if (-self.x).hypot(self.y).abs() > self.deadzone {
            Some((angle / slice_angle) as u8)
        } 
        else { None }
    }

    pub fn update(&mut self) -> Option<Action> {
        if let Some(ev) = self.gilrs.next_event() {
            match ev.event {
                EventType::AxisChanged(axis, data, _code) => {
                    match axis {
                        Axis::RightStickX => { self.x = data.into() },
                        Axis::RightStickY => { self.y = (-data).into() },
                        _ => ()
                    }
                    None
                },
                EventType::ButtonPressed(btn, _code) => {
                    if let Some(slice) = self.get_slice() {
                        println!("{}", slice);
                        match btn {
                            piButton::DPadUp => Some(Action::Press(Direction::South(slice))),
                            piButton::DPadRight => Some(Action::Press(Direction::East(slice))),
                            piButton::DPadDown => Some(Action::Press(Direction::North(slice))),
                            piButton::DPadLeft => Some(Action::Press(Direction::West(slice))),
                            _ => Some(Action::Press(Direction::Other(btn)))
                        }
                    }
                    else { None }
                },
                EventType::ButtonReleased(btn, _code) => {
                    if let Some(slice) = self.get_slice() {
                        println!("{}", slice);
                        match btn {
                            piButton::DPadUp => Some(Action::Release(Direction::South(slice))),
                            piButton::DPadRight => Some(Action::Release(Direction::East(slice))),
                            piButton::DPadDown => Some(Action::Release(Direction::North(slice))),
                            piButton::DPadLeft => Some(Action::Release(Direction::West(slice))),
                            _ => Some(Action::Release(Direction::Other(btn)))
                        }
                    }
                    else { None }
                },
                _ => None
            }
        }
        else { None }
    }
}