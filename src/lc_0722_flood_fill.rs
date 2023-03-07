struct Cell(usize, usize);
use std::collections::VecDeque;

pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let init_cell = Cell(sr as usize, sc as usize);

        let init_color = image[init_cell.0][init_cell.1];
        if init_color == color {
            return image;
        }
        let height = image.len();
        let width = image[0].len();
        let mut image = image;
        
        let mut queue: VecDeque<Cell> = VecDeque::from([init_cell]);

        while let Some(Cell(row, col)) = queue.pop_front() {
            
            let same_color = image[row][col] == init_color;
            if !same_color {
                continue;
            }
            image[row][col] = color;
            if row > 0 {
                queue.push_back(Cell(row - 1, col));
            }
            if row + 1 < height {
                queue.push_back(Cell(row + 1, col));
            }
            if col > 0 {
                queue.push_back(Cell(row, col - 1));
            }
            if col + 1 < width {
                queue.push_back(Cell(row, col + 1));
            }
        }

        image
}

#[cfg(test)]
mod test {
        use super::*;
        
        #[test]
        fn ext1() {
                let image = vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]];
                let sr = 1; let sc = 1; let color = 2;
                let target_output = vec![vec![2,2,2], vec![2,2,0], vec![2,0,1]];

                assert_eq!(target_output, flood_fill(image, sr, sc, color));
        }

        #[test]
        fn ext2() {
                let image = vec![vec![0,0,0], vec![0,0,0]];
                let sr = 0; let sc = 0; let color = 0;
                let target_output = vec![vec![0,0,0], vec![0,0,0]];

                assert_eq!(target_output, flood_fill(image, sr, sc, color));
        }
}