#[macro_use]
extern crate log;
extern crate log4rs;

mod controllers;
pub mod model;

use std::sync::mpsc::channel;
use controllers::terminal::Terminal;
use model::{State, GameState};
use model::game_mode::GameMode;
use std::time::{Duration, Instant};

const ONE_MILLION : f64 = 1000000.;

fn main() {
    initialize_logging();
    //create the number of ms we anticipate each frame to take.
    let target_frame_amt : f64 = duration_to_f64_ms(Duration::new(1, 0).checked_div(144)
        .expect("should never fail. please report if so"));
    let mut terminal = Terminal::new();
    let mut state = State { mode: GameMode::Menu };
    let mut previous = Instant::now();
    let mut lag : f64 = 0.;
    let mut cmd_buffer = vec!();
    let mut keep_running = GameState::Continue;
    trace!("Expecting each frame to take {} ms", target_frame_amt);
    while keep_running == GameState::Continue {
        let start = Instant::now();
        let elapsed = start - previous;
        previous = start;
        lag += duration_to_f64_ms(elapsed);
        while lag > target_frame_amt  {
            trace!("current elapsed time {}", lag);
            keep_running = state.update(cmd_buffer.pop());
            lag -= target_frame_amt;
        }
        let ms_since_frame = lag / target_frame_amt;
        trace!("rendering with ms since: {}", ms_since_frame);
        terminal.render(&state, &ms_since_frame );
        if let Some(cmd) = terminal.get_input(&state) {
            cmd_buffer.push(cmd);
        }
    }
    info!("Exiting run");
}

fn initialize_logging() {
    match log4rs::init_file("config/log4rs.json", Default::default()) {
        Err(e) => {
            println!("Error initializing logging, aborting.");
            panic!("Error initializing logging: ".to_owned() + &e.to_string());
        },
        _ => ()
    }
}

fn duration_to_f64_ms(duration: Duration) -> f64 {
    let seconds = duration.as_secs();
    let nanosecs = duration.subsec_nanos();
    //we definitely lose some precision here,
    //but since our calculations are a liiiiiiiittle bit off due to
    //rendering, the hope is that this is just part of the problem
    //not exacerbating it
    let mut decimal_secs : f64 = nanosecs as f64 / ONE_MILLION;
    decimal_secs += (seconds as f64 * 1000.);
    decimal_secs
}