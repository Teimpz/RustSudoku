use crate::sudoku::util;

#[derive(Copy, Clone)]
pub struct Field
{
	open_values: [bool; util::SUDOKU_SIZE],
}

impl Default for Field{
	fn default() -> Field{
		Field{
			open_values: [true; util::SUDOKU_SIZE],
		}
	}
}

impl std::fmt::Debug for Field {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.get_value(){
			Some(value) => write!(f, "{}", value),
			None => write!(f, ".")
			
			//Some(value) => write!(f, "{}|{}", value, self.get_open_values().count()),
			//None => write!(f, ".|{}", self.get_open_values().count())
		}
	}
}

impl Field{
	pub fn num_open_values(&self) -> usize{
		self.open_values.iter().filter(|&val|*val).count()
	}
	
	pub fn assign_value(&mut self, value: usize){
		if value == 0{
			self.open_values = [true; util::SUDOKU_SIZE];
		}
		else{
			self.open_values = [false; util::SUDOKU_SIZE];
			self.open_values[value-1] = true;
		}
	}
	
	pub fn erase_open_value(&mut self, value: usize) -> bool{
		let value_was_open = self.open_values[value-1];
		self.open_values[value-1] = false;		
		return value_was_open;
	}
	
	pub fn get_value(&self) -> Option<usize>{
		let mut value: Option<usize> = None;
		for index in 0..util::SUDOKU_SIZE{
			if self.open_values[index]{
				if value == None {
					value = Some(index + 1);
				}
				else {
					return None;
				}
			}
		}
		value
	}
	
	pub fn get_open_values(&self) -> impl Iterator<Item=usize> + '_ {
		self.open_values.iter().enumerate().filter_map(|(index, is_open)| match is_open{
			true => Some(index + 1),
			false => None,
		})
	}
}