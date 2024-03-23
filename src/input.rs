use piston::{Button, Key};

use crate::Game;

#[derive(Copy, Clone)]
pub struct KeyState {
    key: Button,
    ticks_since_repeat: usize
}

pub enum EventType {
    Press,
    Release
}

impl Game<'_> {
    pub fn handle_input(&mut self, key: Button, event_type: EventType) {
        match event_type {
            EventType::Press => {
                self.key_states.push(KeyState {
                    key,
                    ticks_since_repeat: 1000
                });
            },
            EventType::Release => {
                self.key_states.retain(|&key_state| key_state.key != key);
            },
        }
    }

    pub fn tick_input(&mut self) {
        let mut key_states = self.key_states.clone();
        for key_state in &mut key_states {
            let bind = find_bind(&key_state.key);
            match bind {
                Ok(bind) => {
                    if key_state.ticks_since_repeat < bind.repeat_every {
                        key_state.ticks_since_repeat += 1;
                    } else {
                        key_state.ticks_since_repeat = 0;
                        self.execute_action(&bind.action);
                    }
                },
                Err(_) => {
                    println!("{:?} key not bound", key_state.key);
                },
            }
        }
        self.key_states = key_states;
    }

    pub fn execute_action(&mut self, action: &BindActions) {
        match action {
            BindActions::MoveLeft => self.board.move_left(),
            BindActions::MoveRight => self.board.move_right(),
            BindActions::MoveDown => self.board.gravity(),
            BindActions::RotateLeft => self.board.rotate_left(),
            BindActions::RotateRight => self.board.rotate_right(),
        }
    }
}

fn find_bind(key: &Button) -> Result<&BindEntry, ()> {
    BINDS.iter().filter(|bind| bind.key == *key).next().ok_or(())
}

static BINDS: [BindEntry; 5] = [
    BindEntry {
        key: Button::Keyboard(Key::A),
        repeat_every: 4,
        action: BindActions::MoveLeft
    },
    BindEntry {
        key: Button::Keyboard(Key::D),
        repeat_every: 4,
        action: BindActions::MoveRight
    },
    BindEntry {
        key: Button::Keyboard(Key::S),
        repeat_every: 1,
        action: BindActions::MoveDown
    },
    BindEntry {
        key: Button::Keyboard(Key::Q),
        repeat_every: 5,
        action: BindActions::RotateLeft
    },
    BindEntry {
        key: Button::Keyboard(Key::E),
        repeat_every: 5,
        action: BindActions::RotateRight
    },
];

enum BindActions {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateLeft,
    RotateRight
}

struct BindEntry {
    key: Button,
    repeat_every: usize,
    action: BindActions
}
