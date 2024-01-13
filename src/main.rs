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
mod game;


fn get_mouse_event(column: u16, row: u16) -> event::Event {
    Mouse(MouseEvent{kind: MouseEventKind::Up(Left), 
        column, row, modifiers: KeyModifiers::empty()})
}


fn main() -> io::Result<()> {

    let mut game_status = game::Game::new();
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    let mut lines = String::new();
    lines = lines + &format!("\r\n\t---------------------------");
    lines = lines + &format!("\r\n\t{}'s turn", game_status.player());
    lines = lines + &format!("\r\n\tClick in any of the cells (1 to 9)!!!");
    lines = lines + &format!("\r\n\tPresss Esc anytime to exit!!!");

    stdout
        .execute(terminal::SetTitle("Tic Tac Toe")).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap()
        .execute(cursor::SavePosition).unwrap()
        .execute(cursor::Hide).unwrap()
        .execute(style::Print(game_status.pretty_print())).unwrap()
        .execute(style::Print(lines)).unwrap()
        .execute(event::EnableMouseCapture)?;

    let mut index = 10;
    loop {


        if event::poll(time::Duration::from_millis(500))? {
            let input = event::read()?;

            if input == event::Event::Key(event::KeyCode::Esc.into()) {
                break;
            }

                 
            if input == get_mouse_event(9, 2) || 
                input == get_mouse_event(10, 2) || 
                input == get_mouse_event(11, 2) {
                index = 0;

            } else if input == get_mouse_event(13, 2) || 
                input == get_mouse_event(14, 2) || 
                input == get_mouse_event(15, 2) {
                index = 1;
            } else if input == get_mouse_event(17, 2) || 
                input == get_mouse_event(18, 2) || 
                input == get_mouse_event(19, 2) {
                index = 2;
            } else if input == get_mouse_event(9, 4) || 
                input == get_mouse_event(10, 4) || 
                input == get_mouse_event(11, 4) {
                index = 3;
            } else if input == get_mouse_event(13, 4) || 
                input == get_mouse_event(14, 4) || 
                input == get_mouse_event(15, 4) {
                index = 4;
            }else if input == get_mouse_event(17, 4) || 
                input == get_mouse_event(18, 4) || 
                input == get_mouse_event(19, 4) {
                index = 5;
            } else if input == get_mouse_event(9, 6) || 
                input == get_mouse_event(10, 6) || 
                input == get_mouse_event(11, 6) {
                index = 6;
            } else if input == get_mouse_event(13, 6) || 
                input == get_mouse_event(14, 6) || 
                input == get_mouse_event(15, 6) {
                index = 7;
            } else if input == get_mouse_event(17, 6) || 
                input == get_mouse_event(18, 6) || 
                input == get_mouse_event(19, 6) {
                index = 8;
            }
            

            if game_status.is_valid_index(index) {
                game_status.update_matrix(index);
                match game_status.status() {
                    0 => {
                        stdout
                            .execute(cursor::RestorePosition).unwrap()
                            .execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap()
                            .execute(style::Print(game_status.pretty_print_win())).unwrap()
                            .execute(style::Print(format!("\r\n\t---------------------------"))).unwrap()
                            .execute(style::Print(format!("\r\n\tCongratulations! {} won!", game_status.player()))).unwrap()
                            .execute(style::Print(format!("\r\n\tPresss Esc to exit!!!"))).unwrap()
                            .execute(cursor::Show).unwrap()
                            .execute(event::DisableMouseCapture).unwrap();

                    }
                    1 => {

                        stdout
                            .execute(cursor::RestorePosition).unwrap()
                            .execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap()
                            .execute(style::Print(game_status.pretty_print())).unwrap()
                            .execute(style::Print(format!("\r\n\t---------------------------"))).unwrap()
                            .execute(style::Print(format!("\r\n\tGame Drew!"))).unwrap()
                            .execute(style::Print(format!("\r\n\tPresss Esc to exit!!!"))).unwrap()
                            .execute(cursor::Show).unwrap()
                            .execute(event::DisableMouseCapture).unwrap();


                    }
                    _ => {

                        game_status.update_index();
                        let mut lines = String::new();
                        lines = lines + &format!("\r\n\t---------------------------");
                        lines = lines + &format!("\r\n\t{}'s turn", game_status.player());
                        lines = lines + &format!("\r\n\tClick in any of the cells (1 to 9)!!!");
                        lines = lines + &format!("\r\n\tPresss Esc anytime to exit!!!");

                        stdout
                            .execute(cursor::RestorePosition).unwrap()
                            .execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap()
                            .execute(style::Print(game_status.pretty_print())).unwrap()
                            .execute(style::Print(lines)).unwrap();
                    }
                }
            } 
        }
    }

    terminal::disable_raw_mode()?;
    stdout
       .execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

