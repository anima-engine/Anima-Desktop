// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glium::glutin::MouseButton;

use super::Intermediate;
use super::super::InputEvent;
use super::super::IntermediateEvent;

/// A `struct` that converts cursor events to button events.
pub struct Button {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pressed: bool
}

impl Button {
    /// Creates a rectangular `Button` with ID `id`.
    pub fn new(id: u32, x: i32, y: i32, width: i32, height: i32) -> Button {
        Button {
            id: id,
            x: x,
            y: y,
            width: width,
            height: height,
            pressed: false
        }
    }

    fn inside(&self, x: i32, y: i32) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;

        0 <= dx && dx <= self.width &&
        0 <= dy && dy <= self.height
    }
}

impl<'a> Intermediate for &'a mut Button {
    fn process(self, input: Vec<InputEvent>) -> Vec<InputEvent> {
        input.into_iter().filter_map(|event| {
            match event {
                InputEvent::Intermediate(
                    IntermediateEvent::CursorPressed(x, y, MouseButton::Left)
                ) if self.inside(x, y) => {
                    self.pressed = true;

                    Some(InputEvent::Intermediate(IntermediateEvent::ButtonPressed(self.id)))
                },
                InputEvent::Intermediate(
                    IntermediateEvent::CursorPressed(_, _, MouseButton::Left)
                ) if self.pressed => {
                    Some(InputEvent::Intermediate(IntermediateEvent::ButtonPressed(self.id)))
                },
                InputEvent::Intermediate(
                    IntermediateEvent::CursorReleased(x, y, MouseButton::Left)
                ) if self.pressed => {
                    self.pressed = false;

                    if self.inside(x, y) {
                        Some(InputEvent::Intermediate(IntermediateEvent::ButtonReleased(self.id)))
                    } else {
                        Some(InputEvent::Intermediate(IntermediateEvent::ButtonCanceled(self.id)))
                    }
                },
                event => Some(event)
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use glium::glutin::MouseButton;

    use super::Button;
    use super::super::Intermediate;
    use super::super::super::{InputEvent, IntermediateEvent};

    #[test]
    fn click_outside() {
        let events = vec![
            InputEvent::Intermediate(
                IntermediateEvent::CursorPressed(10, 50, MouseButton::Left)
            )
        ];
        let mut button = Button::new(3, 40, 40, 20, 20);

        let events = button.process(events);

        match events[0] {
            InputEvent::Intermediate(
                IntermediateEvent::CursorPressed(10, 50, MouseButton::Left)
            ) => assert!(true),
            _ => assert!(false)
        };
    }

    #[test]
    fn click_inside() {
        let events = vec![
            InputEvent::Intermediate(
                IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
            )
        ];
        let mut button = Button::new(3, 40, 40, 20, 20);

        let events = button.process(events);

        match events[0] {
            InputEvent::Intermediate(IntermediateEvent::ButtonPressed(3)) => assert!(true),
            _ => assert!(false)
        };

        let events = vec![
            InputEvent::Intermediate(
                IntermediateEvent::CursorReleased(50, 50, MouseButton::Left)
            )
        ];

        let events = button.process(events);

        match events[0] {
            InputEvent::Intermediate(IntermediateEvent::ButtonReleased(3)) => assert!(true),
            _ => assert!(false)
        };
    }

    #[test]
    fn click_canceled() {
        let events = vec![
            InputEvent::Intermediate(
                IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
            )
        ];
        let mut button = Button::new(3, 40, 40, 20, 20);

        let events = button.process(events);

        match events[0] {
            InputEvent::Intermediate(IntermediateEvent::ButtonPressed(3)) => assert!(true),
            _ => assert!(false)
        };

        let events = vec![
            InputEvent::Intermediate(
                IntermediateEvent::CursorReleased(10, 50, MouseButton::Left)
            )
        ];

        let events = button.process(events);

        match events[0] {
            InputEvent::Intermediate(IntermediateEvent::ButtonCanceled(3)) => assert!(true),
            _ => assert!(false)
        };
    }
}
