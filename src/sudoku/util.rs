pub const SUDOKU_SIZE: usize = 9;

pub fn sudoku_field_indices() -> impl Iterator<Item=(usize,usize)>{
	(0..SUDOKU_SIZE).flat_map(|row| std::iter::repeat(row).zip(0..SUDOKU_SIZE))
}