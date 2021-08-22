use std::{
    io::{stdout, Error, Stdout, Write},
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
    agent::{AgentState, Tag},
    world::Board,
};

pub struct Output {
    screen: HideCursor<AlternateScreen<RawTerminal<Stdout>>>,
    board: Board,
    terminal_size: (u16, u16),
    drawn_positions: Vec<(u16, u16)>,
    scroll: (i32, i32),
    last_ups: Vec<u32>,
    last_fps: Vec<u32>,
    tick: u8,
}

impl Output {
    pub fn new(board: Board) -> Result<Self, Error> {
        let mut output = Self {
            screen: HideCursor::from(AlternateScreen::from(stdout().into_raw_mode()?)),
            board,
            terminal_size: terminal_size()?,
            drawn_positions: Vec::new(),
            scroll: (1, 1),
            last_ups: repeat(0).take(10).collect(),
            last_fps: repeat(0).take(10).collect(),
            tick: 0,
        };

        output.draw_borders();
        Ok(output)
    }

    fn after_scrolling<'sim>(&mut self, states: impl Iterator<Item = (u64, &'sim AgentState)>) {
        self.drawn_positions.clear();
        Self::clear();
        self.draw_borders();
        self.draw_players(states);
        print!("{}{}", color::Reset.fg_str(), cursor::Goto(39, 1));
    }

    pub fn scroll_up<'sim>(&mut self, states: impl Iterator<Item = (u64, &'sim AgentState)>) {
        self.scroll.1 += 1;
        self.after_scrolling(states);
    }

    pub fn scroll_down<'sim>(&mut self, states: impl Iterator<Item = (u64, &'sim AgentState)>) {
        self.scroll.1 -= 1;
        self.after_scrolling(states);
    }

    pub fn scroll_left<'sim>(&mut self, states: impl Iterator<Item = (u64, &'sim AgentState)>) {
        self.scroll.0 += 1;
        self.after_scrolling(states);
    }

    pub fn scroll_right<'sim>(&mut self, states: impl Iterator<Item = (u64, &'sim AgentState)>) {
        self.scroll.0 -= 1;
        self.after_scrolling(states);
    }

    fn position_to_pixel(&self, x: impl Into<i32>, y: impl Into<i32>) -> Option<(u16, u16)> {
        let x = x.into() + self.scroll.0 + 1;
        let y = y.into() + self.scroll.1 + 1;
        if x > 0
            && x < i32::from(self.terminal_size.0)
            && y > 0
            && y + 1 < i32::from(self.terminal_size.1)
        {
            // `x` and `y` are guaranteed to be greater than 0 and smaller than the terminal size, which is u16
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Some((x as u16, y as u16))
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        x: impl Into<i32>,
        y: impl Into<i32>,
        ch: char,
        color: Option<&'static str>,
    ) {
        if let Some((x, y)) = self.position_to_pixel(x, y) {
            if let Some(color) = color {
                print!("{}{}{}", cursor::Goto(x, y), color, ch);
            } else {
                print!("{}{}", cursor::Goto(x, y), ch);
            }
        }
    }

    pub fn clear() {
        print!("{}", clear::All);
    }

    #[allow(
        clippy::similar_names,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_lossless,
        clippy::cast_sign_loss
    )]
    pub fn draw_time(
        &mut self,
        calc_time: Duration,
        draw_time: Duration,
        step: u32,
    ) -> Result<(), Error> {
        let ups = 1_000_000_f64 / (calc_time.as_micros().clamp(1, u128::MAX) as f64 / step as f64);
        let fps = 1_000_000 / draw_time.as_micros().clamp(1, u128::MAX);

        self.last_ups[self.tick as usize % 10] = ups as u32;
        self.last_fps[self.tick as usize % 10] = fps as u32;
        self.tick += 1;

        let avg_ups = self.last_ups.iter().sum::<u32>() / 10;
        let avg_fps = self.last_fps.iter().sum::<u32>() / 10;
        write!(
            self.screen,
            "{}{}tps: {:4} ups ({:4} on avg), fps: {:4} fps ({:4} on avg) {}",
            color::Reset.fg_str(),
            cursor::Goto(1, self.terminal_size.1),
            ups as u32,
            avg_ups as u32,
            fps,
            avg_fps,
            cursor::Goto(39, 1),
        )
    }

    /// Draws the player onto the board]
    pub fn draw_players<'sim>(&mut self, states: impl Iterator<Item = (u64, &'sim AgentState)>) {
        // self.draw_borders()?;
        for (x, y) in &self.drawn_positions {
            if *x < self.terminal_size.0 && *y + 1 < self.terminal_size.1 {
                if let Some((x, y)) = self.position_to_pixel(*x, *y) {
                    print!("{} ", cursor::Goto(x, y));
                }
            }
        }
        self.drawn_positions.clear();
        for (_id, state) in states {
            let x = state.position[0];
            let y = state.position[1];
            self.drawn_positions.push((x, y));
            match state.tag {
                Tag::It => {
                    self.draw(x, y, '@', Some(color::Red.fg_str()));
                }
                Tag::Recent => {
                    self.draw(x, y, '%', Some(color::Yellow.fg_str()));
                }
                Tag::None => {
                    self.draw(x, y, '#', Some(color::Green.fg_str()));
                }
            }
        }
    }

    /// Draws the borders of the ... board
    pub fn draw_borders(&mut self) {
        print!("{}", color::Reset.fg_str());
        self.draw(-1, -1, '╔', None);
        for w in 0..self.board.width {
            self.draw(w, -1, '═', None);
            self.draw(w, self.board.height, '═', None);
        }
        self.draw(self.board.width, -1, '╗', None);

        self.draw(-1, self.board.height, '╚', None);
        for h in 0..self.board.height {
            self.draw(-1, h, '║', None);
            self.draw(self.board.width, h, '║', None);
        }
        self.draw(self.board.width, self.board.height, '╝', None);

        print!(
            "{} q: Quit, t: Update, h/j/k/l: Scroll ",
            cursor::Goto(3, 1)
        );
    }
}
