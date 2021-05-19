// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// key:
// 0: empty
// 1: food
// 2: snake0head
// 3: snake0body
// 4: snake0tail
// 5: snake1head
// 6: snake2body
// 7: snake2tail
// etc.

// i guess i'll need some kind of time lib
// my main loop will record the start time of each turn
// then we'll do whatever thinking for the board itself
// (check elimination, spawn food)
// then we'll do whatever thinking for the snakes
// then we'll sleep until a full second has passed
// then we'll blit the screen
// repeat


// TODO(skend): make a board class that will show all the state we need
struct Board {
    // first is width, second is height
    grid: Vec<Vec<char>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board { grid: vec![vec!['#'; width]; height] }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // for each row, print each character then print newline
        for row in self.grid.clone() {
            for cell in row.clone() {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let board = Board::new(12, 12);
    println!("{}", board);
}
