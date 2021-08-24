use std::{
    io::{stdout, Error, Stdout},
    iter::repeat,
    time::Duration,
};

use termion::{
    clear, color,
    cursor::{self, HideCursor},
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
    terminal_size,
};

use crate::{
    agent::{AgentState, Position, Tag},
    world::Board,
};

/// Simple abstraction over a pixel.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pixel {
    pub x: u16,
    pub y: u16,
}

impl Pixel {
    pub fn new(x: impl Into<u16>, y: impl Into<u16>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

/// Draws the board and the agent on the terminal.
pub struct Output {
    _screen: HideCursor<AlternateScreen<RawTerminal<Stdout>>>,
    board: Board,
    terminal_size: (u16, u16),
    drawn_positions: Vec<Pixel>,
    scroll: (i16, i16),
    last_ups: Vec<u32>,
    last_draw_times: Vec<Duration>,
    tick: u8,
}

impl Output {
    /// Creates an output to draw agents to the terminal.
    pub fn new(board: Board) -> Result<Self, Error> {
        let mut output = Self {
            _screen: HideCursor::from(AlternateScreen::from(stdout().into_raw_mode()?)),
            board,
            terminal_size: terminal_size()?,
            drawn_positions: Vec::new(),
            scroll: (1, 1),
            last_ups: repeat(0).take(10).collect(),
            last_draw_times: repeat(Duration::default()).take(10).collect(),
            tick: 0,
        };

        output.draw_borders();
        Ok(output)
    }

    fn after_scrolling(&mut self, states: &[AgentState]) {
        self.drawn_positions.clear();
        Self::clear();
        self.draw_borders();
        self.draw_players(states);
        print!("{}{}", color::Reset.fg_str(), cursor::Goto(39, 1));
    }

    /// Scroll the board up
    pub fn scroll_up(&mut self, states: &[AgentState]) {
        self.scroll.1 = self.scroll.1.saturating_add(1);
        self.after_scrolling(states);
    }

    /// Scroll the board down
    pub fn scroll_down(&mut self, states: &[AgentState]) {
        self.scroll.1 = self.scroll.1.saturating_sub(1);
        self.after_scrolling(states);
    }

    /// Scroll the board to the left
    pub fn scroll_left(&mut self, states: &[AgentState]) {
        self.scroll.0 = self.scroll.0.saturating_add(1);
        self.after_scrolling(states);
    }

    /// Scroll the board to the right
    pub fn scroll_right(&mut self, states: &[AgentState]) {
        self.scroll.0 = self.scroll.0.saturating_sub(1);
        self.after_scrolling(states);
    }

    fn position_to_pixel(&self, p: Position) -> Option<Pixel> {
        let x = p.x + self.scroll.0 as f32 + 1.;
        let y = p.y + self.scroll.1 as f32 + 1.;
        if x > 0.
            && x < self.terminal_size.0 as f32
            && y > 0.
            && y + 1. < self.terminal_size.1 as f32
        {
            // `x` and `y` are guaranteed to be greater than 0 and smaller than the terminal size, which is u16
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Some(Pixel::new(x as u16, y as u16))
        } else {
            None
        }
    }

    fn draw(&mut self, position: Position, ch: char, color: Option<&'static str>) {
        if let Some(Pixel { x, y }) = self.position_to_pixel(position) {
            if let Some(color) = color {
                print!("{}{}{}", cursor::Goto(x, y), color, ch);
            } else {
                print!("{}{}", cursor::Goto(x, y), ch);
            }
        }
    }

    /// Clears the terminal
    pub fn clear() {
        print!("{}", clear::All);
    }

    /// Draws the timing for updates and frames
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::similar_names
    )]
    pub fn draw_time(&mut self, calc_time: Duration, draw_time: Duration, step: u32) {
        let ups = 1_000_000_f64 / (calc_time.as_micros().clamp(1, u128::MAX) as f64 / step as f64);
        // let draw_time = draw_time.as_millis();

        self.last_ups[self.tick as usize % 10] = ups as u32;
        self.last_draw_times[self.tick as usize % 10] = draw_time;
        self.tick += 1;

        let avg_ups = self.last_ups.iter().sum::<u32>() / 10;
        let avg_draw_times = self.last_draw_times.iter().sum::<Duration>() / 10;
        print!(
            "{}{}{:4} ups ({:4} on avg), frame time: {:4?} ({:4?} on avg) {}",
            color::Reset.fg_str(),
            cursor::Goto(1, self.terminal_size.1),
            ups as u32,
            avg_ups as u32,
            draw_time,
            avg_draw_times,
            cursor::Goto(39, 1),
        );
    }

    /// Draws the player onto the board
    pub fn draw_players(&mut self, states: &[AgentState]) {
        for Pixel { x, y } in &self.drawn_positions {
            print!("{} ", cursor::Goto(*x, *y));
        }
        self.drawn_positions.clear();
        for state in states {
            if let Some(px) = self.position_to_pixel(state.position) {
                self.drawn_positions.push(px);
                match state.tag {
                    Tag::It(_) => {
                        print!("{}{}@", cursor::Goto(px.x, px.y), color::Red.fg_str());
                    }
                    Tag::Recent => {
                        print!("{}{}%", cursor::Goto(px.x, px.y), color::Yellow.fg_str());
                    }
                    Tag::None => print!("{}{}#", cursor::Goto(px.x, px.y), color::Green.fg_str()),
                }
            }
        }
    }

    /// Draws the borders of the ... board
    pub fn draw_borders(&mut self) {
        print!("{}", color::Reset.fg_str());
        self.draw(Position::new(-1., -1.), '╔', None);
        for w in 0..self.board.width {
            self.draw(Position::new(w, -1.), '═', None);
            self.draw(Position::new(w, self.board.height), '═', None);
        }
        self.draw(Position::new(self.board.width, -1.), '╗', None);

        self.draw(Position::new(-1., self.board.height), '╚', None);
        for h in 0..self.board.height {
            self.draw(Position::new(-1., h), '║', None);
            self.draw(Position::new(self.board.width, h), '║', None);
        }
        self.draw(
            Position::new(self.board.width, self.board.height),
            '╝',
            None,
        );

        print!(
            "{} q: Quit, t: Update, h/j/k/l: Scroll ",
            cursor::Goto(3, 1)
        );
    }
}
