use crate::character_stream::*;

use crate::token::*;

use std::env;
use std::fs;
use std::process;

pub struct Scanner{
    // cstream: CharStream,
    pub tokens: Vec<Token>,
    cur_index: usize,
    // token_count: i32,
}

impl Scanner
{
    
    pub fn new(cs: CharStream) -> Scanner
    {
        Scanner{
            // cstream: cs,
            tokens: Scanner::scan_tokens(cs),
            cur_index: 0,
            // token_count: 0;
        }
    }

    fn scan_tokens(mut cs: CharStream) -> Vec<Token>
    {
        let mut token_list: Vec<Token> = vec![];

        // for tracking the token status
        let mut token_count = 0;
        let mut new_token = "";
        let mut adding_new = true;
        let mut stw_letter = false;
        let mut stw_digit = false;
        let mut stw_neg_sign = false;
        let mut is_decimal = false;

        // Tracking down line number and char pos
        let mut char_pos = 0;
        let mut line_num = 0;
        let a = "float".to_string();
        // if (Scanner::is_keyword(a))
        // {
        //     println!("oh yeah");
        // }
        let mut temp_count = 0;
        let mut token_count = 0;
        
        while (cs.more_available())
        {
            let mut token_string = String::from("");
            if (cs.peek_next_char().unwrap() == '-')
            {
                println!("-");
            }
            while (!Scanner::ends_token(cs.peek_next_char().unwrap()) && !Scanner::is_single_operator(cs.peek_next_char().unwrap(), cs.peek_ahead_char(1)))
            {
                if (adding_new)
                {
                    // println!("adding new {}", token_string);
                    let temp_char = cs.get_next_char().unwrap();
                    if (temp_char == '_' || Scanner::is_alphabet(temp_char))
                    {
                        // println!("printing char");
                        stw_letter = true;
                        adding_new = false;
                    } 
                    else if (Scanner::is_digit(temp_char))
                    {
                        stw_digit = true;
                        adding_new = false;
                    }
                    else if (temp_char == '-')
                    {
                        if (!stw_neg_sign)
                        {
                            stw_neg_sign = true;
                        } 
                        else
                        {
                            // eprintln!("Invalid token: {} at line {} pos {}", "--", line_num, char_pos);
                            // process::exit(1);
                            // double negative sign, invalid
                        }
                    }
                    token_string.push(temp_char);
                    // char_pos += 1;
                }
                else
                {
                    
                    if (stw_letter)
                    {
                        let temp_char = cs.get_next_char().unwrap();
                        if(Scanner::is_alphabet(temp_char) || Scanner::is_digit(temp_char) || temp_char == '_')
                        {
                            token_string.push(temp_char);
                            // char_pos += 1;
                        } 
                        else
                        {
                            token_string.push(temp_char);
                            eprintln!("Invalid token: {} at line {} pos {}", token_string, line_num, char_pos);
                            process::exit(1);
                        }
                    }
                    else if (stw_digit)
                    {
                        let temp_char = cs.get_next_char().unwrap();

                        if (Scanner::is_digit(temp_char))
                        {
                            token_string.push(temp_char);
                            // char_pos += 1;
                        } 
                        else if (temp_char == '.')
                        {
                            if (!is_decimal)
                            {
                                is_decimal = true;
                                token_string.push(temp_char);
                                // char_pos += 1;
                            }
                            else
                            {
                                // invalid input, double decimal point
                                token_string.push(temp_char);
                                eprintln!("Invalid token: {} at line {} pos {}", token_string, line_num, char_pos);
                                process::exit(1);
                            }
                        }
                        else 
                        {
                            // report invalid input
                            token_string.push(temp_char);
                            eprintln!("Invalid token: {} at line {} pos {}", token_string, line_num, char_pos);
                            process::exit(1);
                        }
                    }
                    else 
                    {
                        // this part need to be checked 
                        // let temp_char = cs.get_next_char().unwrap();
                        // println!("this unknown char is added to the string {}", temp_char);
                        // token_string.push(temp_char);
                    }
                }

                
            }

            if (!adding_new)
            {
                if (stw_letter)
                {
                    if (Scanner::is_keyword(&token_string))
                    {  
                        // let token_length = token_string.len();
                        let tt = TokenType::KEYWORD;
                        let new_token = Token::new(token_string.to_string(), tt, line_num, char_pos);
                        token_list.push(new_token);
                        // char_pos += token_length;
                    }
                    else
                    {
                        // all identifiers are catogorized as identifier for now
                        // let token_length = token_string.len();
                        let tt = TokenType::IDENTIFIER;
                        let new_token = Token::new(token_string.to_string(), tt, line_num, char_pos);
                        token_list.push(new_token);
                        // char_pos += token_length;
                    }
                }
                else if (stw_digit)
                {
                    if (is_decimal)
                    {
                        // let token_length = token_string.len();
                        let tt = TokenType::FLOATCONSTANT;
                        let new_token = Token::new(token_string.to_string(), tt, line_num, char_pos);
                        token_list.push(new_token);
                        // char_pos += token_length;
                    } 
                    else
                    {
                        // let token_length = token_string.len();
                        let tt = TokenType::INTCONSTANT;
                        let new_token = Token::new(token_string.to_string(), tt, line_num, char_pos);
                        token_list.push(new_token);
                        // char_pos += token_length;
                    }
                }
                // println!("4 {}", token_string);
                char_pos += token_string.len() as i32;
                token_count += 1;
                // println!("what is next {}", cs.peek_next_char().unwrap());
            } 
            
            let mut temp = cs.get_next_char().unwrap();
            // println!("is next {}", temp);
            if (Scanner::is_single_operator(temp, cs.peek_ahead_char(0)))
            {
                // println!("is op {}", temp);
                if (!cs.more_available())
                {
                    // add in as a operator (single char)
                    let tt = TokenType::OPERATOR;
                    let new_token = Token::new(temp.to_string(), tt, line_num, char_pos);
                    token_list.push(new_token);

                    // println!("3 {}", temp);
                    char_pos += 1;
                    token_count += 1;
                    continue;
                }
                let next = cs.peek_next_char().unwrap();
                let mut temp_op = String::from(temp);
                temp_op.push(next);
                if (Scanner::is_double_operator(&temp_op))
                {
                    let mut temp1 = cs.get_next_char().unwrap();

                    let tt = TokenType::OPERATOR;
                    let new_token = Token::new(temp_op.to_string(), tt, line_num, char_pos);
                    token_list.push(new_token);
                    
                    // println!("2 {}", temp_op);
                    char_pos += 1;
                    token_count += 1;
                    // adding this token as a operator token (double char)
                }
                else 
                {
                    if (temp == '!')
                    {
                        // single ! is invalid as input give error
                        eprintln!("Invalid token {} at line {} pos {}", temp, line_num, char_pos);
                        process::exit(1);
                        
                    }
                    // adding this token as a operator token (single char)
                    let tt = TokenType::OPERATOR;
                    let new_token = Token::new(temp.to_string(), tt, line_num, char_pos);
                    token_list.push(new_token);

                    // println!("1 {}", temp);
                    char_pos += 1;
                    token_count += 1;
                }
            }
            else if (Scanner::ends_token(temp))
            {
                if (temp == ' ')
                {
                    // add char pos
                    char_pos += 1;
                } 
                else if (temp == '\n')
                {
                    // add line num
                    line_num += 1;
                    char_pos = 0;
                } 
                else 
                {
                    char_pos += 2;
                }
            }
            // Reset for new token
            adding_new = true;
            stw_letter = false;
            stw_digit = false;
            stw_neg_sign = false;
            is_decimal = false;
            temp_count += 1;
            // token_string = String::from("");
        }
        /* token can be seperated by operators, which is a problem need to deal with the special case of of negative sign as well */
        // println!("There are {} tokens", token_list.len());
        // Scanner::check_tokens(&token_list);
        token_list
        
    }

    // utility

    fn check_tokens(tokens: &Vec<Token>)
    {
        for item in tokens.iter()
        {
            println!("text: {}", item.get_text());
        	println!("token type: {}", item.get_type().as_str());
        	println!("line numer: {}", item.get_line_number());
        	println!("char position: {}", item.get_char_pos());
        }
    }

    pub fn check_tokens2(&self)
    {
        for item in self.tokens.iter()
        {

            println!("text: {}", item.get_text());
        	println!("token type: {}", item.get_type().as_str());
        	println!("line numer: {}", item.get_line_number());
        	println!("char position: {}", item.get_char_pos());
        }
    }
    

    fn is_alphabet(c: char) -> bool
    {
        if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') 
        {
            return true;
        }
        false
    }

    fn is_digit(c: char) -> bool
    {
        if (c >= '0' && c <= '9') 
        {
            return true;
        }
        false
    }

    fn ends_token(c: char) -> bool
    {
        if (c == ' ' || c == '\n' || Scanner::is_single_operator(c, None) || c == '\t')
        {
            true
        }
        else{
            false
        }
    }

    fn is_keyword(s: &String) -> bool
    {
        let keywords: Vec<String> = vec!["unsigned".to_string(), "char".to_string(), "short".to_string(),
         "int".to_string(), "long".to_string()
        , "float".to_string(), "double".to_string(),
        "while".to_string(), "if".to_string(), "return".to_string(), "void".to_string(), "main".to_string()];
        if keywords.contains(&s)
        {
            true
        } else
        {
            false
        }
    }

    fn is_single_operator(c: char, next: Option<char>) -> bool
    {
        let single_operators: Vec<char> = vec!['(', ',', ')', '{', '}', '=',  '<', '>', 
          '!', '+','-', '*', '/', ';'];
         if (single_operators.contains(&c))
         {
             if (c == '-')
             {
                 if (next != None)
                 {
                    if (Scanner::is_digit(next.unwrap()))
                    {
                        return false;
                    }
                    else
                    {
                        return true;
                    }
                 }
                 else 
                 {
                     true
                 }
                 
             }
             else 
             {
                 true
             }
             
         }
         else{
             false
         }
    }

    fn is_double_operator(op: &String) -> bool
    {
        let double_operators: Vec<String> = vec![">=".to_string(), "!=".to_string(), "==".to_string(), "<=".to_string()];
        if (double_operators.contains(&op))
        {
             true
        }
        else{
             false
        }
    }    

    pub fn get_next_token(&mut self) -> &Token
    {
        self.cur_index += 1;
        &self.tokens[self.cur_index - 1]
    }
    
    pub fn peek_next_token(&self) -> &Token
    {
        &self.tokens[self.cur_index]
    }

    pub fn more_tokens_available(&mut self) -> bool
    {
        if (self.cur_index >= self.tokens.len())
        {
            return false;
        }
        true
    }

    pub fn get_current_index(&mut self) -> usize
    {
        self.cur_index
    }

    pub fn set_current_index(&mut self, index: usize)
    {
        self.cur_index = index;
    }

//     pub fn set_declared_variable_type(&mut self, t: TokenType, text: String, index: usize)
//     {
//         // let mut i = index;
//         for i in index..self.tokens.len()
//         {
//             if (self.tokens[i].get_text() == text)
//             {
//                 self.tokens[i].change_type(t);
//             }
//         }
//     }
}

// ">=", "!=" "==" "<="