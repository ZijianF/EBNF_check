mod character_stream;
use character_stream::*;

mod token;
use token::*;

mod scanner;
use scanner::*;

mod parser;
use parser::*;



fn main() {
	let tt = TokenType::OPERATOR;
	let token = Token::new("+".to_string(), tt, 2, 30);
	println!("text: {}", token.get_text());
	println!("token type: {}", token.get_type().as_str());
	println!("line numer: {}", token.get_line_number());
	println!("char position: {}", token.get_char_pos());

	let mut char_stream = CharStream::new("example1.x");
	let mut scanner = Scanner::new(char_stream);
	let mut parser = Parser::new(scanner);
	// parser.check_tokens2();
	parser.analyze();
	parser.write_output();
	// parser.check_tokens2();
	// scanner.check_tokens2();
	// let mut tokens = scanner.scan_tokens();
	// println!("{}", char_stream.input_length());
	// println!("{}", char_stream.self.input.len());
	// let temp: char = char_stream.peek_next_char();
	// println!("{}", char_stream.peek_next_char().unwrap());
	// // println!("{}", char_stream.peek_ahead_char(4).unwrap());
	// println!("{}", char_stream.peek_next_char().unwrap());
	// println!("{}", char_stream.peek_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());
	// println!("{}", char_stream.get_next_char().unwrap());

	// char_stream.print_input();
}

// let mut temp_cur_index = self.scanner.get_current_index();
        // let mut variable_index = temp_cur_index;
        // while (true)
        // {
        //     if (self.scanner.more_tokens_available())
        //     {
        //         let mut temp_next_token = self.scanner.get_next_token();
        //         if (temp_next_token.get_type().as_str() == "Identifier")
        //         {
        //             variable_index = self.scanner.get_current_index() - 1;
        //             break;
        //         }
                
        //     } 
        //     else
        //     {
        //         break;
        //     }
        // }
        // self.scanner.set_current_index(temp_cur_index);

		                // self.scanner.tokens[variable_index].change_type(TokenType::FUNCTION);
            // self.scanner.tokens[variable_index].change_type(TokenType::VARIABLE);
