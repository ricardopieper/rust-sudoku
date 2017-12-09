//! Game board logic

/// Size of game board
const SIZE: usize = 9;

/// Stores game board info
pub struct Gameboard {
    /// Stores the content of the cells
    /// 0 is an empty cell
    pub cells: [[u8; SIZE]; SIZE]
}

impl Gameboard {
    /// Creates a new gameboard
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE]
        }
    }

    /// Populates the board with a solution
    pub fn populate_solution(&mut self) {

        let solution : [[u8; SIZE]; SIZE] = [
            [2, 4, 8, 3, 9, 7, 6, 5, 1],
            [3, 7, 5, 1, 6, 2, 8, 9, 4],
            [6, 9, 1, 5, 4, 8, 7, 2, 3],
            [9, 3, 7, 4, 8, 6, 5, 1, 2],
            [1, 2, 4, 9, 7, 5, 3, 8, 6],
            [5, 8, 6, 2, 1, 3, 9, 4, 7],
            [8, 6, 2, 7, 5, 1, 4, 3, 9],
            [7, 1, 9, 8, 3, 4, 2, 6, 5],
            [4, 5, 3, 6, 2, 9, 1, 7, 8]
        ];

        self.cells = solution;
    }


    /// Gets char at position
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None
        })
    }

    /// Sets char at position
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }

    fn has_no_duplicates(slice: &[u8]) -> bool {
        for j in 0..SIZE {
            let compare = |n: &&u8| {
                j as u8 == **n
            };

            let iterator = slice.iter();
            let filtered = iterator.filter(compare);

            if filtered.count() > 1 {
                return false;
            }
        }
        true
    }

    fn check_rows(cells: &[[u8; SIZE]; SIZE]) -> bool {
        for r in 0..SIZE {
            let row = cells[r];
            if !Gameboard::has_no_duplicates(&row) {
                return false;
            }
        }
        true
    }


    fn check_cols(cells: &[[u8; SIZE]; SIZE]) -> bool {
        let mut row: [u8; SIZE] = [0; SIZE];

        for r in 0..SIZE {
            for c in 0..SIZE {
                row[c] = cells[c][r];
            }
            if !Gameboard::has_no_duplicates(&row) {
                return false;
            }
        }
        true
    }

    fn check_quadrants(cells: &[[u8; SIZE]; SIZE]) -> bool {
        let quadrant_final_indexes: [u8; 3] = [2, 5, 8];

        for i in quadrant_final_indexes.iter() {
            for j in quadrant_final_indexes.iter() {

                let mut quadrant: [u8; SIZE] = [0; SIZE];
                let mut idx = 0;

                for col in (*i - 2).. *i + 1 {
                    for row in (*j - 2).. *j + 1 {
                        quadrant[idx] = cells[col as usize][row as usize];
                        idx+= 1;
                    }
                }

                println!("Checking quadrant {}{}", i, j);
                println!("Quad elements are {:?}", quadrant);

                if !Gameboard::has_no_duplicates(&quadrant) {
                    return false;
                }
            }
        }

        true
    }

    /// Checks if the board is solved
    pub fn check_solution(&self) -> bool {
        let cells = &self.cells;

        let rows_are_ok = Gameboard::check_rows(cells);
        println!("rows are ok: {}", rows_are_ok);

        let cols_are_ok = Gameboard::check_cols(cells);
        println!("rows are ok: {}", cols_are_ok);

        let quads_are_ok = Gameboard::check_quadrants(cells);
        println!("quads are ok: {}", quads_are_ok);

        return rows_are_ok && cols_are_ok && quads_are_ok;
    }
}
