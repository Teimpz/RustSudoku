#[macro_use(s)]
extern crate ndarray;

mod sudoku;

#[allow(dead_code)]
const EXAMPLE_0: [[usize;sudoku::SUDOKU_SIZE];sudoku::SUDOKU_SIZE] =
	[[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0],];

#[allow(dead_code)]													  
const EXAMPLE_1: [[usize;sudoku::SUDOKU_SIZE];sudoku::SUDOKU_SIZE] = 
	[[5,3,0,0,7,0,0,0,0],
	[6,0,0,1,9,5,0,0,0],
	[0,9,8,0,0,0,0,6,0],
	[8,0,0,0,6,0,0,0,3],
	[4,0,0,8,0,3,0,0,1],
	[7,0,0,0,2,0,0,0,6],
	[0,6,0,0,0,0,2,8,0],
	[0,0,0,4,1,9,0,0,5],
	[0,0,0,0,8,0,0,7,9],];

#[allow(dead_code)]													  
const EXAMPLE_2: [[usize;sudoku::SUDOKU_SIZE];sudoku::SUDOKU_SIZE] = 
	[[1,0,0,4,8,9,0,0,6],
	[7,0,0,0,0,0,0,4,0],
	[0,0,0,0,0,1,2,9,5],
	[0,0,7,1,2,0,6,0,0],
	[5,0,0,7,0,3,0,0,8],
	[0,0,6,0,9,5,7,0,0],
	[9,1,4,6,0,0,0,0,0],
	[0,2,0,0,0,0,0,3,7],
	[8,0,0,5,1,2,0,0,4],];

#[allow(dead_code)]
const EXAMPLE_3: [[usize;sudoku::SUDOKU_SIZE];sudoku::SUDOKU_SIZE] = 
	[[0,7,0,0,4,5,0,0,0],
	[9,2,0,0,0,0,1,0,0],
	[0,0,5,0,0,8,0,0,7],
	[0,0,0,0,0,0,6,9,0],
	[0,0,0,7,0,0,0,0,0],
	[0,1,0,4,5,0,0,0,0],
	[2,0,0,0,0,0,4,6,0],
	[0,0,0,0,0,0,0,0,1],
	[6,9,0,0,2,0,0,3,0],];													

fn main() {
	let board_as_array = ndarray::arr2(&EXAMPLE_3);
    let board = sudoku::Board::make_board(&board_as_array);
	println!("Board 1:\n{:?}\n\n", board);
	
	match sudoku::solve(board){
		Some(solved_board) => println!("Solved Board 1:\n{:?}\n\n", solved_board),
		None => println!("failed to solve board"),
	};
}
