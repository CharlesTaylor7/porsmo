mod alert;
mod cli;
mod error;
mod format;
mod input;
mod pomodoro;
mod prelude;
mod stopwatch;
mod terminal;
mod timer;

use crate::input::{get_event, Command, TIMEOUT};
use crate::pomodoro::PomodoroConfig;
use clap::Parser;
use cli::{Cli, CounterMode, PomoMode};
use pomodoro::PomodoroUI;
use prelude::*;
use std::io::Write;
use stopwatch::StopwatchUI;
use terminal::TerminalHandler;
use timer::TimerUI;

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut terminal = TerminalHandler::new()?;
    let stdout = terminal.stdout();
    match args.mode {
        Some(CounterMode::Stopwatch) => StopwatchUI::default().run_ui(stdout)?,
        Some(CounterMode::Timer { target }) => TimerUI::new(target).run_ui(stdout)?,
        Some(CounterMode::Pomodoro {
            mode: PomoMode::Short,
        }) => PomodoroUI::new(PomodoroConfig::short()).run_ui(stdout)?,
        Some(CounterMode::Pomodoro {
            mode: PomoMode::Long,
        }) => PomodoroUI::new(PomodoroConfig::long()).run_ui(stdout)?,
        Some(CounterMode::Pomodoro {
            mode:
                PomoMode::Custom {
                    work_time,
                    break_time,
                },
        }) => PomodoroUI::new(PomodoroConfig {
            work_time,
            break_time,
        })
        .run_ui(stdout)?,
        None => PomodoroUI::new(PomodoroConfig::short()).run_ui(stdout)?,
    }
    Ok(())
}

pub trait CounterUI: Sized {
    fn show(&mut self, out: &mut impl Write) -> Result<()>;
    fn update(&mut self, command: Command);
    fn run_ui(mut self, out: &mut impl Write) -> Result<()> {
        loop {
            self.show(out)?;
            if let Some(cmd) = get_event(TIMEOUT)?.map(Command::from) {
                match cmd {
                    Command::Quit => break,
                    cmd => self.update(cmd),
                }
            }
        }
        Ok(())
    }
}
