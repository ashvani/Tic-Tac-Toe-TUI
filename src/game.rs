pub struct Game {
    matrix: Vec<char>,
    player: char,
    winning_row: Vec<i32>
}

use crossterm::style::Stylize;
impl Game {

    pub fn player(&self) -> char {
        self.player
    }


    pub fn new() -> Self{
        let value: Vec<char> = [' '].repeat(9);
        Self{
            matrix: value,
            player: 'X',
            winning_row: Vec::new()
        }
    }

    fn printline(value: Vec<char>) -> String{
        return format!("\t| {} | {} | {} |\n", value[0], value[1], value[2]);
    }

    pub fn print(&self) -> String {

        let mut printlines = String::new();
        printlines = printlines + &format!("\t{}-----------{}\n", "o", "o");
        for i in 0..3 {
            let row = &self.matrix[((i) * 3)..((i + 1) * 3)];
            printlines = printlines + &Game::printline(row.to_vec()); 
            if i < 2 {
                printlines = printlines + &format!("\t ----------- \n");
            } else {
                printlines = printlines + &format!("\t{}-----------{}\n", "o", "o"); 
            }
        }

        printlines
    }


    pub fn pretty_print(&self) {

        let boundry_line = format!("\t{}-----------{}", "o", "o");
        print!("{}", boundry_line);
        for i in 0..9 {
            if i % 3 == 0 && i > 0{
                print!("\n\t ----------- \n\t|");
            } else if i % 3 == 0 {
                print!("\n\t|");
            }

            if self.winning_row.iter().any(|&j| j == i) {
                print!(" {} |", &self.matrix[i as usize].to_string().green());
            } else {
                print!(" {} |", &self.matrix[i as usize]);
            }

       }
        println!("\n{}", boundry_line);
    }

    pub fn is_valid_index(&self, index: usize) -> bool {
       if index < 9 && self.matrix[index] == ' ' {
           true 
       } else {
           false 
       }
    }

    pub fn update_matrix(& mut self, index: usize) {
        self.matrix[index] = self.player;
    }

    pub fn update_index(&mut self) {
        self.player = if self.player == 'X' {'O'} else {'X'};
    }


    fn row_win(&mut self) -> bool {

        if self.matrix[0] != ' ' && (self.matrix[0] == self.matrix[1] && 
            self.matrix[0] == self.matrix[2]) {
            self.winning_row = vec![0, 1, 2];
            true 
        } else if self.matrix[3] != ' ' && (self.matrix[3] == self.matrix[4] && 
            self.matrix[3] == self.matrix[5])  {
            self.winning_row = vec![3, 4, 5];
            true
        } else if  self.matrix[6] != ' ' && (self.matrix[6] == self.matrix[7] && 
            self.matrix[6] == self.matrix[8]) {
            self.winning_row = vec![6, 7, 8];
            true 
        } else {
            false
        }

    }

    fn column_win(&mut self) -> bool {

        if self.matrix[0] != ' ' && (self.matrix[0] == self.matrix[3] &&
                                     self.matrix[0] == self.matrix[6]) {
            self.winning_row = vec![0, 3, 6];
            true 
        } else if self.matrix[1] != ' ' && (self.matrix[1] == self.matrix[4] &&
                                     self.matrix[1] == self.matrix[7]) {
            self.winning_row = vec![1, 4, 7];
            true 
        } else if self.matrix[2] != ' ' && (self.matrix[2] == self.matrix[5] &&
                                     self.matrix[2] == self.matrix[8]) {
            self.winning_row = vec![2, 5, 8];
            true 
        } else {
            false 
        }

    }

    fn diagonal_win(&mut self) -> bool {
        if self.matrix[0] != ' ' && (self.matrix[0] == self.matrix[4] &&
                                     self.matrix[0] == self.matrix[8]) {
            self.winning_row = vec![0, 4, 8];
            true 
        } else if self.matrix[2] != ' ' && (self.matrix[2] == self.matrix[4] &&
                                            self.matrix[2] == self.matrix[6]) {
            self.winning_row = vec![2, 4, 6];
            true 
        } else {
            false 
        }

    }


    fn win(&mut self) -> bool {
        if self.row_win() || self.column_win() || self.diagonal_win() {
            true
        } else {
            false 
        }
    }

    fn draw(&self) -> bool {
       for val in &self.matrix {
           if val == &' ' {
               return false
           }

       }
       true
    }


    // Check if current value of matrix results in win (0), draw (1) or
    // incomplete (2)
    //
    pub fn status(&mut self) -> u8 {
        if self.win() {
            0
        } else if self.draw() {
            1
        } else {
            2
        }
    }



}


