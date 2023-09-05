
/* uses one HashSet to check rules */
// use std::collections::HashSet;
// pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
//         let mut decisions: HashSet<String> = HashSet::with_capacity(729);
//         for row in 0..9 {
//                 for column in 0..9 {
//                         let entry = board[row][column];
//                         if entry != '.' {
//                                 let region = 3 * (row / 3) + (column / 3);
//                                 let valid_row    = decisions.insert(format!("r{}{}", row, entry));
//                                 let valid_column = decisions.insert(format!("c{}{}", column, entry));
//                                 let valid_region = decisions.insert(format!("z{}{}", region, entry));
//                                 if !(valid_row && valid_column && valid_region) {
//                                         return false;
//                                 }
//                          }
//                 }
//         }
//         true
// }
use std::collections::HashSet;
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut    rows: HashSet<(usize, char)> = HashSet::with_capacity(81);
        let mut columns: HashSet<(usize, char)> = HashSet::with_capacity(81);
        let mut regions: HashSet<(usize, char)> = HashSet::with_capacity(81);
        for r_i in 0..9 {
                for c_i in 0..9 {
                        let entry = board[r_i][c_i];
                        if entry != '.' {
                                let    valid_row = rows.insert((r_i, entry));
                                let    valid_col = columns.insert((c_i, entry));
                                let valid_region = regions.insert((3 * (r_i / 3) + (c_i / 3), entry));
                                if !(valid_row && valid_col && valid_region) {
                                        return false
                                }
                         }
                }
        }
        true
}

#[cfg(test)]
mod tests {
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
