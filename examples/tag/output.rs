use std::{
    io::{stdout, Error, Stdout, Write},
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
    drawn_pixels: Vec<(u16, u16)>,
}

impl Output {
    pub fn new(board: Board) -> Result<Self, Error> {
        let mut output = Self {
            screen: HideCursor::from(AlternateScreen::from(stdout().into_raw_mode()?)),
            board,
            terminal_size: terminal_size()?,
            drawn_pixels: Vec::new(),
        };
        output.draw_borders()?;

        Ok(output)
    }

    fn draw(&mut self, x: u16, y: u16, ch: char, color: Option<&'static str>) -> Result<(), Error> {
        if x < self.terminal_size.0 && y + 1 < self.terminal_size.1 {
            if let Some(color) = color {
                write!(self.screen, "{}{}{}", cursor::Goto(x + 1, y + 1), color, ch)
            } else {
                write!(self.screen, "{}{}", cursor::Goto(x + 1, y + 1), ch)
            }
        } else {
            Ok(())
        }
    }

    fn clear(&mut self) -> Result<(), Error> {
        write!(self.screen, "{}", clear::All)
    }

    pub fn draw_time(&mut self, calc_time: Duration, draw_time: Duration) -> Result<(), Error> {
        let calc_time_ms = calc_time.as_millis().clamp(1, u128::MAX);
        let draw_time_ms = draw_time.as_millis().clamp(1, u128::MAX);
        write!(
            self.screen,
            "{}{}, tps: {:4} ups, fps: {:4} fps ",
            color::Reset.fg_str(),
            cursor::Goto(22, 1),
            1000 / calc_time_ms,
            1000 / draw_time_ms,
        )
    }

    /// Draws the player onto the board]
    pub fn draw_players<'sim>(
        &mut self,
        states: impl Iterator<Item = (u64, &'sim AgentState)>,
    ) -> Result<(), Error> {
        // self.draw_borders()?;
        for (x, y) in &self.drawn_pixels {
            if *x < self.terminal_size.0 && *y + 1 < self.terminal_size.1 {
                write!(
                    self.screen,
                    "{}{} ",
                    cursor::Goto(x + 1, y + 1),
                    color::Reset.fg_str()
                )?;
            }
        }
        self.drawn_pixels.clear();
        for (_id, state) in states {
            let x = state.position[0] + 1;
            let y = state.position[1] + 1;
            self.drawn_pixels.push((x, y));
            match state.tag {
                Tag::It => {
                    self.draw(x, y, '@', Some(color::Red.fg_str()))?;
                }
                Tag::Recent => {
                    self.draw(x, y, '%', Some(color::Yellow.fg_str()))?;
                }
                Tag::None => {
                    self.draw(x, y, '#', Some(color::Green.fg_str()))?;
                }
            }
        }
        Ok(())
    }

    /// Draws the borders of the ... board
    fn draw_borders(&mut self) -> Result<(), Error> {
        self.clear()?;
        self.draw(0, 0, '╔', None)?;
        for w in 1..=self.board.width {
            self.draw(w, 0, '═', None)?;
            self.draw(w, self.board.height + 1, '═', None)?;
        }
        self.draw(self.board.width + 1, 0, '╗', None)?;

        self.draw(0, self.board.height + 1, '╚', None)?;
        for h in 1..=self.board.height {
            self.draw(0, h, '║', None)?;
            self.draw(self.board.width + 1, h, '║', None)?;
        }
        self.draw(self.board.width + 1, self.board.height + 1, '╝', None)?;

        write!(self.screen, "{} q: Quit, t: Update ", cursor::Goto(3, 1))?;

        self.screen.flush()?;

        Ok(())
    }

    /// Get a mutable reference to the output's screen.
    pub fn screen(&mut self) -> &mut impl Write {
        &mut self.screen
    }
}
