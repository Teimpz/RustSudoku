pub const SUDOKU_SIZE: usize = 9;
pub const SUDOKU_SIZE_SQUARED: usize = SUDOKU_SIZE * SUDOKU_SIZE;

pub fn sudoku_field_indices() -> impl Iterator<Item=(usize,usize)>{
	(0..SUDOKU_SIZE).flat_map(|row| std::iter::repeat(row).zip(0..SUDOKU_SIZE))
}