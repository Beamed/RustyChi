pub mod map;
pub mod game_mode;
pub mod messaging;

use self::messaging::{Event, Message};
use self::game_mode::GameMode;

#[derive(Eq, Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    Continue,
    Exit,
}

pub struct State {
    pub mode: GameMode
}

impl State {
    pub fn update(&mut self, message: Option<Message>) -> GameState {
        if let Some(msg) = message {
            debug!("Received message: {}", msg.msg);
            match msg.evt {
                Event::ModeSelected(mode) => return self.update_mode(mode),
            }
        };
        GameState::Continue
        //else run update
    }

    fn update_mode(&mut self, mode: GameMode) -> GameState {
        match mode {
            GameMode::Quit => GameState::Exit,
            _ => {
                self.mode = mode;
                GameState::Continue
            }//do_stuff
        }
    }
}