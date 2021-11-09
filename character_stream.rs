use std::fs::File;
use std::io;
use std::convert::TryFrom;
use std::io::prelude::*;


pub struct CharStream {
	input: Vec<u8>,
	current_index: usize,
}

impl CharStream {

	pub fn new(f: &str) -> CharStream {
		CharStream {
			input: std::fs::read(f).expect("Unable to read file"),
			current_index: 0,
		}
	}
	
	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
		if(self.current_index >= self.input.len() - 1)
		{
			// let a = 'a';
			// return (a < 'b') as bool;
			return false;
		}
		true
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available. 
	pub fn peek_next_char(&self) -> Option<char> {
		if(self.more_available())
		{
			Some(self.input[self.current_index] as char)
		}
		else 
		{
			None
		}
		
	}

	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	pub fn peek_ahead_char(&self, k: i32) -> Option<char> {
		if (self.current_index + (k as usize)) < self.input.len()
		{
			Some(self.input[self.current_index + (k as usize)] as char)
		}
		else
		{
			None
		}
		
	}

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {
		if(self.more_available())
		{
			self.current_index += 1;
			Some(self.input[self.current_index-1] as char)
			
		}
		else 
		{
			None
		}
	}

	/* utilities */
	pub fn input_length(&self) -> usize{
		self.input.len()
	}

	pub fn print_input(&self)
	{
		let input_string = String::from_utf8(self.input.clone());
		println!("{:?}", input_string);
	}
	/* utilities */
}



