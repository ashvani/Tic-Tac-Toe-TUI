use std::io::{self, stdout};
use crossterm::{
    ExecutableCommand,
    cursor,
    style,
    terminal,
    event
};


use::crossterm::event::{KeyModifiers, Event::Mouse, MouseEvent, MouseEventKind, MouseButton::Left};

use std::time;

fn pretty_print() -> String {

    let pattern = format!(
"\r\n\to-----------o\
\r\n\t| {} | {} | {} |\
\r\n\t-------------\
\r\n\t| {} | {} | {} |\
\r\n\t-------------\
\r\n\t| {} | {} | {} |\
\r\n\to-----------o\n",
     "X", "X", "X",
     "X", "X", "X", "X", "X", "X");
    pattern


}



fn main() -> io::Result<()> {

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout
        .execute(terminal::SetTitle("Tic Tac Toe")).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap()
        .execute(cursor::Hide).unwrap()
        .execute(style::Print(pretty_print())).unwrap()
        .execute(event::EnableMouseCapture)?;

    loop {
        if event::poll(time::Duration::from_millis(500))? {
            let input = event::read()?;

            if input == event::Event::Key(event::KeyCode::Esc.into()) {
                break;
            }

            if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 10, row: 2, 
                modifiers: KeyModifiers::empty()}) {
                print!("\r\nthis is that");
                print!("\r\nWoooooohoooooooooooooo!!!!");
            }



        } 
    }

    stdout
        .execute(cursor::Show).unwrap()
        .execute(event::DisableMouseCapture).unwrap()
        .execute(terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;
    Ok(())
}

