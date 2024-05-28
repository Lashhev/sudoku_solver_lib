use nalgebra::SMatrix;
use std::collections::HashSet;
use std::fs;

pub type SudokuGrid = SMatrix<u8, 9, 9>;
type Values = HashSet<u8>;
pub struct SudokuSolver {
}

impl SudokuSolver {
    const MAX_ITER: u32 = 1000u32;


    pub fn solve(puzzle: &SudokuGrid) -> Option<SudokuGrid> {
        if !SudokuSolver::check_input(puzzle)
        {
            return None;
        }
        let mut solution = puzzle.clone();
        if SudokuSolver::solve_imp(&mut solution) {
            Some(solution)
        } else {
            None
        }
    }
    /**
     * Doesn't check if an input sudoku puzzle is valid.(it has only unique numbers in each row, column and block)
     */
    pub fn solve_unchecked(puzzle: &SudokuGrid) -> Option<SudokuGrid> {
        let mut solution = puzzle.clone();
        if SudokuSolver::solve_imp(&mut solution) {
            Some(solution)
        } else {
            None
        }
    }

    fn check_input(puzzle: &SudokuGrid) -> bool
    {
        for row_index in 0..9 {
            if !SudokuSolver::check_row(puzzle, row_index){
                return false;
            }
        }

        for column_index in 0..9
        {
            if !SudokuSolver::check_cols(puzzle, column_index) {
                return false;
            }
        }

        for i in 0..3
        {
            for j in 0..3
            {
                if !SudokuSolver::check_block(puzzle, i, j)
                {
                    return  false;
                }
            }
        }
        
        true
    }

    fn check_row(puzzle: &SudokuGrid, row_index: usize) -> bool {
        let mut values = Values::new();
        for v in &puzzle.row(row_index) {
            if !values.contains(v) || *v == 0u8 {
                values.insert(v.clone());
            }
            else {
                return false;
            }
        }
        true
    }

    fn check_cols(puzzle: &SudokuGrid, col_index: usize) -> bool {
        let mut values = Values::new();
        for v in &puzzle.column(col_index) {
            if !values.contains(v) || *v == 0u8 {
            values.insert(v.clone());
            }
            else {
                return false;
            }
        }
        true
    }

    fn check_block(puzzle: &SudokuGrid, block_row: usize, block_col: usize) -> bool {
        let mut values = Values::new();
        let block_row_start = 3 * block_row;
        let block_column_start = 3 * block_col;
        for r in 0..3 {
            for c in 0..3 {
                let v = puzzle[((block_row_start + r), (block_column_start + c))];
                if !values.contains(&v) || v == 0u8 {
                values.insert(v);
                }
                else {
                    return false;
                }
            }
        }
        true
    }
    
    fn solve_imp(puzzle: &mut SudokuGrid) -> bool {
        let mut min_row: Option<usize> = None;
        let mut min_column: Option<usize> = None;
        let mut min_values: Option<Values> = None;
        // print!("{}", puzzle);
        let mut counter = 0u32;
        while counter < SudokuSolver::MAX_ITER {
            counter = counter +1;
            min_row = None;
            min_values = None;
            min_column = None;
            for row_index in 0..9 {
                for column_index in 0..9 {
                    if puzzle[(row_index, column_index)] != 0 {
                        continue;
                    }
                    let possible_values =
                        SudokuSolver::find_possible_values(puzzle, row_index, column_index);
                    let possible_vaue_count = possible_values.len();
                    if possible_vaue_count == 0 {
                        return false;
                    }
                    if possible_vaue_count == 1 {
                        puzzle[(row_index, column_index)] = *possible_values.iter().next().unwrap();
                    }
                    if min_values.is_none()
                        || possible_vaue_count < min_values.as_ref().unwrap().len()
                    {
                        min_row = Some(row_index);
                        min_column = Some(column_index);
                        min_values = Some(possible_values);
                    }
                }
            }
            if min_values.is_none() {
                return true;
            } else if 1 < min_values.as_ref().unwrap().len() {
                break;
            }
        }
        if counter >= SudokuSolver::MAX_ITER
        {
            return false;
        }
        for v in min_values.unwrap() {
            let mut puzzle_copy = puzzle.clone();
            puzzle_copy[(min_row.unwrap(), min_column.unwrap())] = v;
            if SudokuSolver::solve_imp(&mut puzzle_copy) {
                puzzle.copy_from(&puzzle_copy);
                return true;
            }
        }
        false
    }
    
    fn find_possible_values(puzzle: &SudokuGrid, row_index: usize, column_index: usize) -> Values {
        let mut values = Values::from_iter(1..10);
        let in_rows = SudokuSolver::get_row_values(puzzle, row_index);
        let in_cols = SudokuSolver::get_cols_values(puzzle, column_index);
        let in_block = SudokuSolver::get_block_values(puzzle, row_index, column_index);
        values.retain(|x| !(in_rows.contains(x) || in_cols.contains(x) || in_block.contains(x)));
        // println!("{:?}", values);
        values
    }

    fn get_row_values(puzzle: &SudokuGrid, row_index: usize) -> Values {
        let mut values = Values::new();
        for v in &puzzle.row(row_index) {
            values.insert(v.clone());
        }
        values
    }

    fn get_cols_values(puzzle: &SudokuGrid, col_index: usize) -> Values {
        let mut values = Values::new();
        for v in &puzzle.column(col_index) {
            values.insert(v.clone());
        }
        values
    }

    fn get_block_values(puzzle: &SudokuGrid, row_index: usize, column_index: usize) -> Values {
        let mut values = Values::new();
        let block_row_start = 3 * (row_index / 3);
        let block_column_start = 3 * (column_index / 3);
        for r in 0..3 {
            for c in 0..3 {
                values.insert(puzzle[((block_row_start + r), (block_column_start + c))]);
            }
        }
        values
    }

    pub fn save(path: String, puzzle: &SudokuGrid) {
        fs::write(path, serde_json::to_string(puzzle).unwrap()).expect("Can't write to file");
    }

    pub fn load(path: String) -> SudokuGrid {
        let puzzle = {
        let res: String = fs::read_to_string(path).expect("Can't read file");
        serde_json::from_str::<SudokuGrid>(&res).unwrap()
        };
        puzzle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::RowSVector;
    type SudokuRow = RowSVector<u8, 9>;
    #[test]
    fn it_works() {
        let puzzle1 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![0, 0, 0, 0, 6, 0, 7, 0, 0]),
            SudokuRow::from_vec(vec![0, 5, 9, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 1, 0, 2, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 1, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![6, 0, 0, 5, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![3, 0, 0, 0, 0, 0, 4, 6, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 9, 1]),
            SudokuRow::from_vec(vec![8, 0, 0, 7, 4, 0, 0, 0, 0]),
        ]);
        let puzzle2 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![5, 0, 6, 0, 0, 7, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 5, 2, 4, 9, 8]),
            SudokuRow::from_vec(vec![0, 2, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 7, 0, 0, 0, 9, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 5, 0, 4, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 1, 0, 0, 6, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 9, 0, 6, 7, 3, 0]),
            SudokuRow::from_vec(vec![0, 0, 9, 0, 7, 0, 0, 6, 0]),
            SudokuRow::from_vec(vec![8, 0, 0, 3, 0, 5, 0, 0, 4]),
        ]);

        let puzzle3 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![5, 0, 7, 0, 0, 7, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 5, 2, 4, 9, 8]),
            SudokuRow::from_vec(vec![0, 2, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 7, 0, 0, 0, 9, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 5, 0, 4, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 1, 0, 0, 6, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 9, 0, 6, 7, 3, 0]),
            SudokuRow::from_vec(vec![0, 2, 9, 0, 7, 0, 0, 6, 0]),
            SudokuRow::from_vec(vec![8, 0, 0, 3, 0, 5, 0, 0, 4]),
        ]);
        let puzzle4 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![0, 0, 7, 0, 0, 7, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 9, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ]);
        let puzzle5 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![0, 0, 1, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 1, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
            SudokuRow::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ]);
        let ref_sol1 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![2, 3, 8, 9, 6, 5, 7, 1, 4]),
            SudokuRow::from_vec(vec![7, 5, 9, 4, 1, 3, 6, 8, 2]),
            SudokuRow::from_vec(vec![4, 1, 6, 2, 7, 8, 9, 5, 3]),
            SudokuRow::from_vec(vec![9, 4, 5, 1, 3, 6, 2, 7, 8]),
            SudokuRow::from_vec(vec![6, 8, 7, 5, 2, 4, 1, 3, 9]),
            SudokuRow::from_vec(vec![3, 2, 1, 8, 9, 7, 4, 6, 5]),
            SudokuRow::from_vec(vec![1, 6, 2, 3, 5, 9, 8, 4, 7]),
            SudokuRow::from_vec(vec![5, 7, 4, 6, 8, 2, 3, 9, 1]),
            SudokuRow::from_vec(vec![8, 9, 3, 7, 4, 1, 5, 2, 6]),
        ]);

        let ref_sol2 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![5, 8, 6, 4, 9, 7, 3, 1, 2]),
            SudokuRow::from_vec(vec![7, 3, 1, 6, 5, 2, 4, 9, 8]),
            SudokuRow::from_vec(vec![9, 2, 4, 8, 3, 1, 5, 7, 6]),
            SudokuRow::from_vec(vec![2, 7, 8, 5, 6, 9, 1, 4, 3]),
            SudokuRow::from_vec(vec![6, 1, 5, 7, 4, 3, 2, 8, 9]),
            SudokuRow::from_vec(vec![4, 9, 3, 1, 2, 8, 6, 5, 7]),
            SudokuRow::from_vec(vec![1, 4, 2, 9, 8, 6, 7, 3, 5]),
            SudokuRow::from_vec(vec![3, 5, 9, 2, 7, 4, 8, 6, 1]),
            SudokuRow::from_vec(vec![8, 6, 7, 3, 1, 5, 9, 2, 4]),
        ]);
        let sol1 = SudokuSolver::solve(&puzzle1);
        assert!(sol1.is_some());
        assert_eq!(sol1.unwrap(), ref_sol1);

        let sol2 = SudokuSolver::solve(&puzzle2);
        assert!(sol2.is_some());
        assert_eq!(sol2.unwrap(), ref_sol2);

        let sol3 = SudokuSolver::solve(&puzzle3);
        assert!(sol3.is_none());

        let sol4 = SudokuSolver::solve(&puzzle4);
        assert!(sol4.is_none());

        let sol5 = SudokuSolver::solve(&puzzle5);
        assert!(sol5.is_none());

    }
    #[test]
    fn io_test() {
        let ref_sol1 = SudokuGrid::from_rows(&[
            SudokuRow::from_vec(vec![2, 3, 8, 9, 6, 5, 7, 1, 4]),
            SudokuRow::from_vec(vec![7, 5, 9, 4, 1, 3, 6, 8, 2]),
            SudokuRow::from_vec(vec![4, 1, 6, 2, 7, 8, 9, 5, 3]),
            SudokuRow::from_vec(vec![9, 4, 5, 1, 3, 6, 2, 7, 8]),
            SudokuRow::from_vec(vec![6, 8, 7, 5, 2, 4, 1, 3, 9]),
            SudokuRow::from_vec(vec![3, 2, 1, 8, 9, 7, 4, 6, 5]),
            SudokuRow::from_vec(vec![1, 6, 2, 3, 5, 9, 8, 4, 7]),
            SudokuRow::from_vec(vec![5, 7, 4, 6, 8, 2, 3, 9, 1]),
            SudokuRow::from_vec(vec![8, 9, 3, 7, 4, 1, 5, 2, 6]),
        ]);
        SudokuSolver::save("data.json".to_string(), &ref_sol1);
    }
}
