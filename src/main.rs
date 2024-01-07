use std::io::{self, stdout};
use crossterm::{
    ExecutableCommand,
    cursor,
    style,
    terminal,
    event
};


use::crossterm::event::{KeyModifiers, Event::Mouse, MouseEvent, MouseEventKind, MouseButton::Left};
use std::{thread, time};
mod game;

fn main() -> io::Result<()> {

    let mut game_status = game::Game::new();
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout
        .execute(terminal::SetTitle("Tic Tac Toe")).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap()
        .execute(cursor::SavePosition).unwrap()
        .execute(cursor::Hide).unwrap()
        .execute(event::EnableMouseCapture)?;

    let mut index = 10;
    loop {
        let mut lines = String::new();
        lines = lines + &format!("\r\n-----------------------------------");
        lines = lines + &format!("\r\n\t{}'s turn", game_status.player());
        lines = lines + &format!("\r\n\tClick in any of the cells (1 to 9)!!!");
        lines = lines + &format!("\r\n\tPresss Esc anytime to exit!!!");


        stdout
            .execute(cursor::RestorePosition).unwrap()
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap()
            .execute(style::Print(game_status.pretty_print())).unwrap()
            .execute(style::Print(lines)).unwrap();


        if event::poll(time::Duration::from_millis(500))? {
            let input = event::read()?;

            if input == event::Event::Key(event::KeyCode::Esc.into()) {
                break;
            }

                 
            if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 10, row: 2, 
                modifiers: KeyModifiers::empty()}) {
                index = 0;

            } else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 14, row: 2, 
                modifiers: KeyModifiers::empty()}) {
                index = 1;
            } else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 18, row: 2, 
                modifiers: KeyModifiers::empty()}) {
                index = 2;
            } else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 10, row: 4, 
                modifiers: KeyModifiers::empty()}) {
                index = 3;
            } else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 14, row: 4, 
                modifiers: KeyModifiers::empty()}) {
                index = 4;
            }else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 18, row: 4, 
                modifiers: KeyModifiers::empty()}) {
                index = 5;
            } else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 10, row: 6, 
                modifiers: KeyModifiers::empty()}) {
                index = 6;
            } else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 14, row: 6, 
                modifiers: KeyModifiers::empty()}) {
                index = 7;
            } else if input == Mouse(MouseEvent{kind: MouseEventKind::Up(Left), column: 18, row: 6, 
                modifiers: KeyModifiers::empty()}) {
                index = 8;
            }
            

            if game_status.is_valid_index(index) {
                game_status.update_matrix(index);
                match game_status.status() {
                    0 => {
                        stdout
                            .execute(cursor::RestorePosition).unwrap()
                            .execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap()
                            .execute(style::Print(game_status.pretty_print())).unwrap()
                            .execute(style::Print(format!("\r\n\tCongratulations! {} won!", game_status.player()))).unwrap()
                            .execute(cursor::Show).unwrap()
                            .execute(event::DisableMouseCapture).unwrap();

                        break;
                    }
                    1 => {

                        stdout
                            .execute(cursor::RestorePosition).unwrap()
                            .execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap()
                            .execute(style::Print(game_status.pretty_print())).unwrap()
                            .execute(style::Print(format!("\r\n\tGame Drew!"))).unwrap()
                            .execute(cursor::Show).unwrap()
                            .execute(event::DisableMouseCapture).unwrap();

                        break;

                    }
                    _ => {
                    }
                }
                game_status.update_index();
            } 
        }
    }

    thread::sleep(time::Duration::from_millis(7000));
    terminal::disable_raw_mode()?;
    stdout
       .execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

