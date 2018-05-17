extern crate easycurses;

use self::easycurses::*;
use self::easycurses::Color::{Blue, White};
use model::game_mode::GameMode;
use model::messaging::{Message, Event};
use model::State;

pub struct Terminal {
    terminal: EasyCurses,

}

//give ourselves a fancy lifetime to swe live as long as any callback
impl Terminal {
    pub fn new() -> Terminal {
        let mut term = EasyCurses::initialize_system().expect("Unable to initialize terminal application.");

        term.set_input_mode(InputMode::Character);
        term.set_title_win32("RustyChi");
        term.set_keypad_enabled(true);
        term.set_input_timeout(TimeoutMode::Immediate);
        term.set_cursor_visibility(CursorVisibility::Invisible);
        term.set_echo(false);

        return Terminal {terminal: term}
    }

    pub fn get_input(&mut self) -> Option<Message> {
        trace!("Fetching input");
        let input= self.terminal.get_input();
        self.translate_menu_input(input)
    }


    pub fn render(&mut self, state: &State, ms_since_frame: &f64) {
        self.render_ui();
        match state.mode {
            GameMode::Menu => {
                self.build_initial_menu()
            },
            GameMode::MapEditor => {
                self.render_map_editor()
            },
            _ => ()
        }
    }

    fn render_ui(&mut self) {
        self.render_bottom_field();
        self.render_right_field();
    }

    pub fn clear(&mut self) {
        self.terminal.clear();
    }

    fn build_initial_menu(&mut self) {
        let options = vec!("[N]ew Game", "[L]oad Game", "[M]ap Editor", "[Q]uit");
        self.render_menu(options);
    }

    fn translate_menu_input(&self, input: Option<Input>) -> Option<Message> {
        if let Some(cmd) = input {
            match cmd {
                Input::Character(c) => {
                    debug!("Input received: {}", c);
                    if c == 'n' || c == 'N' {
                        Some(Message { msg: "Selected New Game".to_string(), evt: Event::ModeSelected(GameMode::New)})
                    } else if c == 'l' || c == 'L' {
                        Some(Message { msg: "Selected Load Game".to_string(), evt: Event::ModeSelected(GameMode::Load)})
                    } else if c == 'm' || c == 'M' {
                        Some(Message {msg: "Selected Map Editor".to_string(), evt: Event::ModeSelected(GameMode::MapEditor)})
                    } else if c == 'q' || c == 'Q' {
                        Some(Message {msg: "Selected quit".to_string(), evt: Event::ModeSelected(GameMode::Quit)})
                    } else {
                        debug!("Unknown key pressed, returning None: {}", c);
                        None
                    }
                },
                _ => None
            }
        } else {
            None
        }

    }

    fn render_map_editor(&mut self) {

    }

    fn render_bottom_field(&mut self) {

        let mut term = &mut self.terminal;
        let (height, width) = term.get_row_col_count();
        let top_boundary = height / 4;//bottom 3rd of view
        for x in 0..width {
            term.move_xy(x, top_boundary);
            term.print_char('-');
        }
    }

    fn render_right_field(&mut self) {
        let mut term = &mut self.terminal;
        let (height, width) = term.get_row_col_count();
        let left_boundary = (3*width) / 4;
        let bottom_boundary = (height / 4) + 1;
        for y in bottom_boundary..height {
            term.move_xy(left_boundary, y);
            term.print_char('|');
        }
    }

    fn render_menu(&mut self, render_options: Vec<&str>) {
        let (height, mut width) = self.get_bottom_xy();
        let max_width = Terminal::get_max_width(&render_options);
        let (num_rows, num_per_row) = Terminal::determine_rows(render_options.len(), width, max_width);
        let num_items = render_options.len() as i32;
        let mut column = 0;
        let mut cur_row = 0;
        let mut term = &mut self.terminal;
        for option in render_options.iter() {
            //add padding
            let col = ((width / num_items) * column) + 5;
            //on row 1:
            let row = (2*height / 3) - cur_row;
            term.move_xy(col, row);
            term.print(option);
            column += 1;
            if column > num_per_row {
                column = 0;
                cur_row += 1;
            }
        }
    }

    fn get_max_width(strings: &Vec<&str>) -> usize {
        let mut cand = 0;
        for string in strings {
            if string.len() > cand  {
                cand = string.len();
            }
        }
        cand
    }

    fn determine_rows(items: usize, width: i32, max_width: usize) -> (i32, i32) {
        //add 4 to extra width to allow spaces
        let optimal_num_per_row = width / (max_width as i32 + 4);
        let remainder = items as i32 % optimal_num_per_row;
        if remainder > 0 {
            return ((items as i32/ optimal_num_per_row) + 1, optimal_num_per_row);
        }
        (items as i32 / optimal_num_per_row, optimal_num_per_row)
    }

    fn get_bottom_xy(&mut self) -> (i32, i32) {
        let (height, width) = self.terminal.get_row_col_count();
        let bottom_start = height / 4;
        (bottom_start, width)
    }
}
