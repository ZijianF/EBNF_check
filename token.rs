pub enum TokenType {
    NONE,
	INTCONSTANT,
	FLOATCONSTANT,
    OPERATOR,
    KEYWORD,
    VARIABLE,
	FUNCTION,
    INVALID,
    IDENTIFIER
}

// let tt = TokenType::OPERATOR;
// 	let token = Token::new("+".to_string(), tt, 2, 30);
// 	println!("text: {}", token.get_text());
// 	println!("token type: {}", token.get_type().as_str());
// 	println!("line numer: {}", token.get_line_number());
// 	println!("char position: {}", token.get_char_pos());

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match &self {
            TokenType::NONE => "None",
            TokenType::INTCONSTANT => "IntConstant",
            TokenType::FLOATCONSTANT => "FloatConstant",
            TokenType::OPERATOR => "Operator",
            TokenType::KEYWORD => "Keyword",
            TokenType::VARIABLE => "Variable",
            TokenType::FUNCTION => "Function",
            TokenType::INVALID => "Invalid",
            TokenType::IDENTIFIER => "Identifier"
            
        }   
    }   
}



pub struct Token {
    text: String,
    token_type: TokenType,
    line_number: i32,
    char_position: i32 
}

impl Token {
    pub fn new(s: String, t: TokenType, linenum: i32, charpos: i32) -> Token {
        Token {
            text: s,
            token_type: t,
            line_number: linenum,
            char_position: charpos
        }   
    }   

    pub fn get_text(&self) -> &str {
        &self.text
    }   

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }   

    pub fn get_line_number(&self) -> i32 {
        self.line_number
    }   

    pub fn get_char_pos(&self) -> i32 {
        self.char_position
    }   

    pub fn change_type(&mut self, t: TokenType)
    {
        self.token_type = t;
    }
}
