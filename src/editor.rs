use std::time::Duration;

use crate::terminal_functions::TerminalFunctions;
use crate::view::View;

use crossterm::{ 
  event::{ 
      read,
      Event::*,
      KeyCode, 
      poll, KeyEvent, KeyModifiers
  },
  Result
};

pub struct Editor {
  terminal: TerminalFunctions,
  view: View
}

impl Editor {

  pub fn new() -> Result<Self> {
    let terminal = TerminalFunctions::new();
    let view = View::new(terminal.win_size);

    Ok(
      Self {
        terminal: terminal,
        view: view
      }
    )

  }

  pub fn run(&mut self) -> Result<bool> {
    self.view.refresh_screen()?;
    self.process_input()
  }

  // Pensar em uma forma de fazer dentro do arquivo de config que vc ainda vai fazer
  fn process_input(&mut self) -> Result<bool>{
    match self.input_event()? {
        KeyEvent {
          code: KeyCode::Char('q'),
          modifiers: KeyModifiers::CONTROL,
          kind,
          state,
        } => return Ok(false),

        KeyEvent {
          code: direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
          modifiers: KeyModifiers::NONE,
          kind,
          state,
        } => self.view.move_cursor(direction),

        _ => {}
    }

    Ok(true)
  } 

  fn input_event(&self) -> Result<KeyEvent> {
    loop {
      if poll(Duration::from_millis(100))? {
        if let Ok(event) = read() {
          if let Key(key_event) = event { 
            return Ok(key_event);
          } 
        }
      }
    }
  }
}