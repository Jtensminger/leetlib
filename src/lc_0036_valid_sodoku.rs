
use std::collections::HashSet;
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set: HashSet<char> = HashSet::with_capacity(9);
        let mut regions: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];
        /* test rows && regions */
        for r_i in 0..9 {
                for c_i in 0..9 {
                        let cell = board[r_i][c_i];
                        if cell != '.' {
                                if !set.insert(cell) || !regions[cell_region(r_i, c_i)].insert(cell) {
                                        return false
                                }
                        }
                }
                set.clear()
        }
        
        /* test columns */
        for c_i in 0..9 {
                for r_i in 0..9 {
                        let cell = board[r_i][c_i];
                        if cell != '.' && !set.insert(cell) {
                                return false
                        }
                }
                set.clear()
        }
        true
}

fn cell_region(r_i: usize, c_i: usize) -> usize {
        match (r_i, c_i) {
                (0..=2, 0..=2) => 0,
                (0..=2, 3..=5) => 1,
                (0..=2, 6..=8) => 2,                                
                (3..=5, 0..=2) => 3,
                (3..=5, 3..=5) => 4,
                (3..=5, 6..=8) => 5,
                (6..=8, 0..=2) => 6,
                (6..=8, 3..=5) => 7,
                _ => 8
        }
}
/* 
        naive: teach each rule seperately:
                for each row:
                        test rule
                for each column:
                        test rule
                for each region:
                        test rule
*/
#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                let board: Vec<Vec<char>> = vec![
                        vec!['5','3','.','.','7','.','.','.','.'],
                        vec!['6','.','.','1','9','5','.','.','.'],
                        vec!['.','9','8','.','.','.','.','6','.'],
                        vec!['8','.','.','.','6','.','.','.','3'],
                        vec!['4','.','.','8','.','3','.','.','1'],
                        vec!['7','.','.','.','2','.','.','.','6'],
                        vec!['.','6','.','.','.','.','2','8','.'],
                        vec!['.','.','.','4','1','9','.','.','5'],
                        vec!['.','.','.','.','8','.','.','7','9'],
                ];
                assert_eq!(true, is_valid_sudoku(board));
        }

        #[test]
        fn ext2() {
                let board: Vec<Vec<char>> = vec![
                        vec!['8','3','.','.','7','.','.','.','.'],
                        vec!['6','.','.','1','9','5','.','.','.'],
                        vec!['.','9','5','.','.','.','.','6','.'],
                        vec!['8','.','.','.','6','.','.','.','3'],
                        vec!['4','.','.','8','.','3','.','.','1'],
                        vec!['7','.','.','.','2','.','.','.','6'],
                        vec!['.','6','.','.','.','.','2','8','.'],
                        vec!['.','.','.','4','1','9','.','.','5'],
                        vec!['.','.','.','.','8','.','.','7','9']
                ];
                /* Same as Example 1, except with the 5 in the top left corner being modified to 8.
                   Since there are two 8's in the top left 3x3 sub-box, it is invalid. */
                assert_eq!(false, is_valid_sudoku(board));
        }
}
