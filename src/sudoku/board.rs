use crate::sudoku::util;

use ndarray;

#[derive(Clone)]
pub struct Board
{
	fields: ndarray::Array3<bool>,
}

impl Default for Board{
	fn default() -> Board{
		Board{
			fields: ndarray::Array::from_elem((util::SUDOKU_SIZE+1,util::SUDOKU_SIZE,util::SUDOKU_SIZE), true),
		}
	}
}

impl std::fmt::Debug for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.get_solved_values())
	}
}

impl Board{		
	fn assign_value(&mut self, row: usize, col: usize, value: usize){
		if value == 0{
			return;
		}
		
		let mut field = self.fields.slice_mut(s![..,row,col]);
		
		field.fill(false);
		field[value] = true;
	}
	
	fn get_solved_values(&self) -> ndarray::Array2<usize>{
		self.fields.map_axis(ndarray::Axis(0), |field| -> usize {
			if !field[0]{
				field.indexed_iter()
					.find(|(_index,&is_open)| is_open)
					.map(|(index,&_is_open)| index)
					.unwrap_or(0)
			}
			else {
				0
			}
		})
	}
	
	fn get_number_of_open_values(&self) -> ndarray::Array2<usize>{
		self.fields.slice(s![1..,..,..]).map_axis(
			ndarray::Axis(0), 
			|field| field.iter().filter(|&val|*val).count())
	}

	fn is_valid(&self) -> bool{		
		util::sudoku_field_indices()
			.map(|(row,col)|self.fields.slice(s![1..,row,col]))
			.all(|field| field.iter().any(|&val| val))
	}
	
	fn is_solved(&self) -> bool{
		self.fields.slice(s![0,..,..]).iter().all(|&val|!val)
	}	
	
	fn get_field_open_values(&self, row: usize, col: usize) -> Vec<usize>{
		self.fields.slice(s![1..,row,col])
			.indexed_iter()
			.filter(|(_index,&is_open)| is_open)
			.map(|(index,_is_open)| index + 1)
			.collect()
	}
	
	fn branch_guess(&self, row: usize, col: usize) -> Vec<Board>{
		let make_guess_assuming_value = |value: &usize| -> Board {
			let mut guess = self.clone();
			guess.assign_value(row,col,*value);
			guess
		};
		
		self.get_field_open_values(row,col)
			.iter()
			.map(make_guess_assuming_value)
			.collect()
	}	

	
	pub fn make_board(values: &ndarray::Array2<usize> ) -> Board {
		let mut board: Board = Board::default();
		
		for (row,col) in util::sudoku_field_indices(){
			board.assign_value(row,col,values[(row,col)]);
		}
		
		return board;
	}
}

fn propagate_final_values(mut board: Board) -> Board{
	let current_solved_values = board.get_solved_values();
	
	for ((row,col),&value) in current_solved_values.indexed_iter(){
		if value != 0{
			board.fields.slice_mut(s![value,row,..]).fill(false);
			board.fields.slice_mut(s![value,..,col]).fill(false);
			board.fields.slice_mut(s![value,(row/3)*3..(row/3)*3+3,(col/3)*3..(col/3)*3+3]).fill(false);
			board.fields[(value,row,col)] = true;
		}
	}
		
	let (mut open_value_indicators, open_values) 
		= board.fields.view_mut().split_at(ndarray::Axis(0),1);	
	
	open_value_indicators.assign(
		&open_values.map_axis(
			ndarray::Axis(0), |field|field.iter().filter(|&val|*val).count() != 1));
	
	if current_solved_values != board.get_solved_values(){
		board = propagate_final_values(board);
	}
	
	board
}

pub fn solve(mut board: Board) -> Option<Board>{
	debug_assert!(board.is_valid());
	
	board = propagate_final_values(board);
	if !board.is_valid(){
		return None;
	}
	
	if board.is_solved(){
		return Some(board);
	}
	
	let ((branch_row,branch_col),_) = board.get_number_of_open_values()
		.indexed_iter()
		.filter(|((_,_),&num_open)| num_open > 1)
		.min_by(|((_,_),num_open_a),((_,_),num_open_b)| num_open_a.cmp(&num_open_b))
		.unwrap();

	for mut branched_board in board.branch_guess(branch_row,branch_col)
	{	
		branched_board = propagate_final_values(branched_board);
		if !branched_board.is_valid(){
			continue;
		}
		
		if branched_board.is_solved(){
			return Some(branched_board);
		}
	
		match solve(branched_board){
			Some(board) => return Some(board),
			None => (),
		};
	}
	
	None
}