use crate::sudoku::util;
use crate::sudoku::field::Field;

#[derive(Clone)]
pub struct Board
{
	fields: [Field; util::SUDOKU_SIZE_SQUARED],
}

impl Default for Board{
	fn default() -> Board{
		Board{
			fields: [Field::default(); util::SUDOKU_SIZE_SQUARED],
		}
	}
}

impl std::fmt::Debug for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in 0..util::SUDOKU_SIZE{
			for col in 0..util::SUDOKU_SIZE{
				write!(f, "{:?}, ", self.get_field(row,col))?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}

impl Board{	
	fn is_valid(&self) -> bool{
		self.fields.iter().all(|field|field.num_open_values() > 0)
	}
	
	fn is_solved(&self) -> bool{
		self.fields.iter().all(|field|field.num_open_values() == 1)
	}	

	fn get_field_mut(&mut self, row: usize, col: usize) -> &mut Field{
		&mut self.fields[row * util::SUDOKU_SIZE + col]
	}
	fn get_field(&self, row: usize, col: usize) -> &Field{
		&self.fields[row * util::SUDOKU_SIZE + col]
	}

	fn for_each_other_field_in_column<F>(&mut self, row: usize, col: usize, func: &mut F) where F: FnMut(&mut Field){
		for row_it in 0..util::SUDOKU_SIZE{
			if row_it != row{
				func(&mut self.get_field_mut(row_it, col));
			}
		}
	}
	
	fn for_each_other_field_in_row<F>(&mut self, row: usize, col: usize, func: &mut F) where F: FnMut(&mut Field){
		for col_it in 0..util::SUDOKU_SIZE{
			if col_it != col{
				func(&mut self.get_field_mut(row,col_it));
			}
		}
	}
	
	fn for_each_other_field_in_square<F>(&mut self, row: usize, col: usize, func: &mut F) where F: FnMut(&mut Field){
		for row_it in (row/3)*3..(row/3)*3+3{
			for col_it in (col/3)*3..(col/3)*3+3{
				if row_it != row || col_it != col{
					func(&mut self.get_field_mut(row_it,col_it));
				}
			}
		}
	}
	
	fn optimize(&mut self){
		let mut changed = false;
		
		for(row,col) in util::sudoku_field_indices(){
			match self.get_field_mut(row,col).get_value(){
				None => (),
				Some(value) => {
					let mut erase_open_value = |field: &mut Field| changed = field.erase_open_value(value) || changed;
					self.for_each_other_field_in_column(row,col,&mut erase_open_value);
					self.for_each_other_field_in_row(row,col,&mut erase_open_value);
					self.for_each_other_field_in_square(row,col,&mut erase_open_value);
				},
			};
		}
		
		if changed{
			self.optimize()
		}
	}
	
	fn branch_guess(&self, row: usize, col: usize) -> Vec<Board>{
		let make_guess_assuming_value = |value: usize|{
			let mut guess = self.clone();
			guess.get_field_mut(row,col).erase_open_value(value);
			guess.optimize();	
			
			match guess.is_valid(){
				true => Some(guess),
				false => None,
			}
		};
		
		self.get_field(row,col)
			.get_open_values()
			.filter_map(make_guess_assuming_value)
			.collect()
	}	
	
	pub fn solve(mut board: Board) -> Option<Board>{
		board.optimize();
		
		if board.is_solved(){
			return Some(board);
		}
		
		let ((row,col),_) = util::sudoku_field_indices()
			.zip(board.fields.iter())
			.filter(|((_,_),field)| field.num_open_values() >= 2)
			.min_by(|((_,_),field_a),((_,_),field_b)| field_a.num_open_values().cmp(&field_b.num_open_values()))
			.unwrap();	
		
		for branched_board in board.branch_guess(row,col)
		{	
			if branched_board.is_solved(){
				return Some(branched_board);
			}

			match Board::solve(branched_board){
				Some(board) => return Some(board),
				None => (),
			};
		}
		
		None
	}
	
	pub fn make_board(values: [[usize;util::SUDOKU_SIZE];util::SUDOKU_SIZE] ) -> Board {
		let mut board: Board = Board::default();
		
		for (row,col) in util::sudoku_field_indices(){
			board.get_field_mut(row,col).assign_value(values[row][col]);
		}
		
		return board;
	}
}