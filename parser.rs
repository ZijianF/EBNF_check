use crate::character_stream::*;

use crate::token::*;

use crate::scanner::*;


use std::env;
use std::fs;
use std::process;

pub struct Parser{
    scanner: Scanner,
}

impl Parser
{
    pub fn new(sc: Scanner) -> Parser
    {
        Parser
        {
            scanner: sc,
        }
    }

    // utilities
    pub fn check_tokens2(&self)
    {
        println!("token#: {}", self.scanner.tokens.len());
        for item in self.scanner.tokens.iter()
        {
            println!("text: {}", item.get_text());
        	println!("token type: {}", item.get_type().as_str());
        	println!("line numer: {}", item.get_line_number());
        	println!("char position: {}", item.get_char_pos());
        }
    }
    

    
    

    fn AddOperator(&mut self) -> bool
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "+" && next_token.get_text() != "-")
        {
            eprintln!("Grammatical Error: AddOperator: invalid AddOperator. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }
        true
    }

    fn MultOperator(&mut self) -> bool
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "*" && next_token.get_text() != "/")
        {
            eprintln!("Grammatical Error: MultOperator: invalid MultOperator. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }
        true
    }

    fn Factor(&mut self) -> bool //done maybe
    {
        let mut current_index = self.scanner.get_current_index();
        if (self.scanner.more_tokens_available())
        {
            if (self.scanner.peek_next_token().get_text() == "(")
            {
                let mut next_token = self.scanner.get_next_token();
                if (!self.Expression())
                {
                    eprintln!("Grammatical Error: Factor: no expression in (). Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                    self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

                    return false;
                }
                next_token = self.scanner.get_next_token();
                if (next_token.get_text() != ")")
                {
                    eprintln!("Grammatical Error: Factor: Expression not followed by ) . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                    self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

                    return false;
                }
                return true;
            } 
            else 
            {
                if (!self.Constant())
                {
                    self.scanner.set_current_index(current_index);
                    if (self.Identifier())
                    {
                        current_index = self.scanner.get_current_index();
                        if (self.scanner.peek_next_token().get_text() == "(")
                        {
                            let mut next_token = self.scanner.get_next_token();
                            if (self.scanner.peek_next_token().get_text() == ")")
                            {
                                next_token = self.scanner.get_next_token();
                                return true;
                            }
                            else if (self.Expression())
                            {
                                current_index = self.scanner.get_current_index();
                                while (true)
                                {
                                    if (self.scanner.peek_next_token().get_text() != ",")
                                    {
                                        break;
                                    }
                                    next_token = self.scanner.get_next_token();

                                    if (!self.Expression())
                                    {
                                        // can report error here
                                        //terminate the program
                                        eprintln!("Grammatical Error: Factor: missing expression after , . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                                        self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
                        
                                        return false;
                                    }
                                    current_index = self.scanner.get_current_index();
                                }
                            }
                            self.scanner.set_current_index(current_index);

                            next_token = self.scanner.get_next_token();
                            if (next_token.get_text() != ")")
                            {
                                // terminate the program, report invalid input no ;
                                eprintln!("Grammatical Error: Factor: missing closing ) . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());                   
                                return false;
                            }
                            return true;
                        }
                        return true;
                        
                    }
                    else 
                    {
                        // self.scanner.set_current_index(current_index);
                        eprintln!("Grammatical Error: Factor: Invlid Factor. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                        self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
        
                        return false;
                    }
                }
                else
                {
                    return true;
                }
            }
        } 
        else
        {
            return false;    
        }  
    }

    fn Term(&mut self) -> bool //done
    {
        if (!self.Factor())
        {
            // eprintln!("Grammatical Error: Term: Missing Factor in the beginning. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        let mut current_index = self.scanner.get_current_index();
        while (true) // optional 
        {
            // first declaration part is optional 
            if (self.MultOperator())
            {
                if (self.Factor())
                {
                    // do nothing
                }
                else
                {
                    eprintln!("Grammatical Error: Term: MultOperator not followed by Factor. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                    self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

                    return false;
                }
            }
            else 
            {
                self.scanner.set_current_index(current_index);
                break;
            }

            current_index = self.scanner.get_current_index();
        } 
        true
    }
    

    fn SimpleExpression(&mut self) -> bool // done
    {
        if (!self.Term())
        {
            // eprintln!("Grammatical Error: SimpleExpression: Missing term in the beginning. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
            return false;
        }

        let mut current_index = self.scanner.get_current_index();
        while (true) // optional 
        {
            // first declaration part is optional 
            if (self.AddOperator())
            {
                if (self.Term())
                {
                    // do nothing
                }
                else
                {
                    eprintln!("Grammatical Error: SimpleExpression: AddOperator not followed by term. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                    self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
        
                    return false;
                }
            }
            else 
            {
                self.scanner.set_current_index(current_index);
                break;
            }

            current_index = self.scanner.get_current_index();
        } 
        true
    }

    fn RelationOperator(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.peek_next_token();
        if (next_token.get_text() != "==" && next_token.get_text() != "<"
        && next_token.get_text() != ">" && next_token.get_text() != "<="
        && next_token.get_text() != ">=" && next_token.get_text() != "!=")
        {
            eprintln!("Grammatical Error: RalationiOperator: invalid operator type. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
            return false;
        }
        next_token = self.scanner.get_next_token();
        true
    }

    fn IntegerType(&mut self) -> bool //done
    {
        if (self.scanner.more_tokens_available())
        {
            let mut next_token = self.scanner.peek_next_token();
            if (next_token.get_text() == "unsigned")
            {
                next_token = self.scanner.get_next_token();
            }
    
            next_token = self.scanner.get_next_token();
            if (next_token.get_text() != "char" && next_token.get_text() != "short"
            && next_token.get_text() != "int" && next_token.get_text() != "long")
            {
                // eprintln!("Grammatical Error: IntegerType: invalid integer type. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
    
                return false;
            }
            true
        }
        else 
        {
            return false;
        }
    }

    fn FloatType(&mut self) -> bool //done
    {
        if (self.scanner.more_tokens_available())
        {
            let mut next_token = self.scanner.get_next_token();
            if (next_token.get_text() != "float" && next_token.get_text() != "double")
            {
                // eprintln!("Grammatical Error: FloatType: invalid float type. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
    
                
                return false;
            }
            true
    
        }
        else
        {
            return false;
        }
    }


    fn Expression(&mut self) -> bool //done
    {
        if (!self.SimpleExpression())
        {
            // eprintln!("Grammatical Error: Expression: Missing simple expression. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        let mut current_index = self.scanner.get_current_index();
        if (self.RelationOperator())
        {
            if (!self.SimpleExpression())
            {
                // invalid
                eprintln!("Grammatical Error: Expression: relatioin operator not followed by simple expression. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
    
                return false;
            }
            else
            {
                // do nothing
            }
        } 
        else 
        {
            current_index = self.scanner.get_current_index();
        }
        true
    }

    fn Parameter(&mut self) -> bool //done
    {
        if (!self.DataType())
        {
            eprintln!("Grammatical Error: Parameter: Missing Datatype. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }
        
        let mut variable_index = self.scanner.get_current_index();;
        
        if (!self.Identifier())
        {
            eprintln!("Grammatical Error: Parameter: Missing identifier . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }
        // self.scanner.tokens[variable_index].change_type(TokenType::VARIABLE);
        // self.scanner.set_declared_variable_type(TokenType::VARIABLE, self.scanner.tokens[variable_index].get_text().to_string(), variable_index);
        for i in variable_index..(self.scanner.tokens.len() - 1)
        {
            if (self.scanner.tokens[i].get_text() == self.scanner.tokens[variable_index].get_text())
            {
                self.scanner.tokens[i].change_type(TokenType::VARIABLE);
            }
        }
        true
    }


    fn Assignment(&mut self) -> bool //done
    {
        if (!self.Identifier())
        {
            // eprintln!("Grammatical Error: Assignment: Assignment not started with identifier . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "=")
        {
            eprintln!("Grammatical Error: Assignment: Identifier not followed by = . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
            return false;
        }

        let mut current_index = self.scanner.get_current_index();
        while (true) // optional 
        {
            // first declaration part is optional 
            if (!self.Identifier())
            {
                self.scanner.set_current_index(current_index);
                break;
            }

            next_token = self.scanner.get_next_token();
            if (next_token.get_text() != "=")
            {
                eprintln!("Grammatical Error: Assignment: = Identifier not followed by = ; . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
                self.scanner.set_current_index(current_index);
                break;
            }
            current_index = self.scanner.get_current_index();
        } 

        if (!self.Expression())
        {
            eprintln!("Grammatical Error: Assignment: No expression in assignment . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        



            return false;
        }

        next_token = self.scanner.get_next_token();
        if (next_token.get_text() != ";")
        {
            eprintln!("Grammatical Error: Assignment: assignment not ended with ; . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        


            return false;
        }
        true
    }

    fn WhileLoop(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "while")
        {
            // eprintln!("Grammatical Error: WhileLoop: statement started with while . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        


            return false;
        }

        next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "(")
        {
            eprintln!("Grammatical Error: WhileLoop: no ( followed after while . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        if (!self.Expression())
        {
            eprintln!("Grammatical Error: WhileLoop: no expression after ( . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        next_token = self.scanner.get_next_token();
        if (next_token.get_text() != ")")
        {
            eprintln!("Grammatical Error: WhileLoop: expression not followed by ) . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        if (!self.Block())
        {
            eprintln!("Grammatical Error: WhileLoop: Loop not followed by a block. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }
        true
    }

    fn IfStatement(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "if")
        {
            // eprintln!("Grammatical Error: IfStatement: statement started with if . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "(")
        {
            eprintln!("Grammatical Error: IfStatement: no ( followed after if . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        if (!self.Expression())
        {
            eprintln!("Grammatical Error: IfStatement: no expression after ( . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        next_token = self.scanner.get_next_token();
        if (next_token.get_text() != ")")
        {
            eprintln!("Grammatical Error: IfStatement: expression not followed by ) . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
            return false;
        }

        if (!self.Block())
        {
            eprintln!("Grammatical Error: IfStatement: statement not followed by a block. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        


            return false;
        }
        true

    }

    fn ReturnStatement(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "return")
        {
            // eprintln!("Grammatical Error: ReturnStatement: return statement no started with return. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
            return false;
        }

        if (!self.Expression())
        {
            eprintln!("Grammatical Error: ReturnStatement: No expression followed return . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }

        next_token = self.scanner.get_next_token();
        if (next_token.get_text() != ";")
        {
            eprintln!("Grammatical Error: ReturnStatement: statement not ended with ; . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        

            return false;
        }
        true
    }

    fn Constant(&mut self) -> bool // done
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_type().as_str() == "IntConstant" || next_token.get_type().as_str() == "FloatConstant")
        {
            return true;
        }
        false
    }

    fn DataType(&mut self) -> bool //done 
    {
        let mut current_index = self.scanner.get_current_index();
        // println!("current index is {}", current_index);
        if (!self.IntegerType())
        {
            self.scanner.set_current_index(current_index);
            if (!self.FloatType())
            {
                // eprintln!("Grammatical Error: DataType: Invalid Datatype. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                return false;
            }
            else
            {
                return true;
            }
        } 
        else 
        {
            return true;
        }
    }

    fn Identifier(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_type().as_str() == "Identifier" ||next_token.get_type().as_str() == "Variable"
         ||next_token.get_type().as_str() == "Function")
        {
            return true;
        }
        false
    }

    fn Statement(&mut self) -> bool //done
    {
        let mut current_index = self.scanner.get_current_index();
        if (!self.Assignment())
        {
            self.scanner.set_current_index(current_index);
            if (!self.WhileLoop())
            {
                self.scanner.set_current_index(current_index);
                if (!self.IfStatement())
                {
                    self.scanner.set_current_index(current_index);
                    if (!self.ReturnStatement())
                    {
                        self.scanner.set_current_index(current_index);
                        let mut next_token = self.scanner.get_next_token();
                        // if (next_token.get_text() != "(")
                        // {
                        //     // terminate
                        //     eprintln!("Grammatical Error: Statement: invalid statement. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                        //     self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
                        //     return false;
                        // }

                        if (!self.Expression())
                        {
                            // terminate
                            // eprintln!("Grammatical Error: Statement: no expression in (). Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
                            return false;
                        }

                        next_token = self.scanner.get_next_token();
                        if (next_token.get_text() != ";")
                        {
                            // terminate
                            eprintln!("Grammatical Error: Statement: no ; follows expression. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
                            return false;
                        }

                        next_token = self.scanner.get_next_token();
                        // if (next_token.get_text() != ")")
                        // {
                        //     // terminate
                        //     eprintln!("Grammatical Error: Statement: expression not ended with ) . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                        //     self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());        
                        //     return false;
                        // }
                        return true;
                    }
                    else
                    {
                        return true;
                    }
                }
                else
                {
                    return true;
                }
            }
            else
            {
                return true;
            }
        }
        else
        {
            return true;
        }
    }

    
    fn DeclarationType(&mut self) -> bool // done
    {
        if (self.DataType())
        {
            if (self.Identifier())
            {
                return true;
            }
            else
            {
                eprintln!("Grammatical Error: DeclarationType: DeclarationType: datatype not followed by Identifer. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                return false;
            }
        }
        else
        {
            // eprintln!("Grammatical Error: DeclarationType: Declaration type not started with data type. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }
    }

    
    fn ParameterBlock(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.get_next_token();        
        if (next_token.get_text() != "(")
        {
            // terminate the program, report invalid input no ;
            eprintln!("Grammatical Error: ParameterBlock: parameter block not started with ( . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());

            return false;
        }

        let mut current_index = self.scanner.get_current_index();
        if (self.Parameter())
        {
            current_index = self.scanner.get_current_index();
            while (true)
            {
                if (self.scanner.peek_next_token().get_text() != ",")
                {
                    break;
                }
                next_token = self.scanner.get_next_token();

                if (!self.Parameter())
                {
                    // can report error here
                    //terminate the program
                    eprintln!("Grammatical Error: ParameterBlock: no parameter follows , in parameter block . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                    self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                    return false;
                }
                current_index = self.scanner.get_current_index();
            }
        }

        self.scanner.set_current_index(current_index);

        next_token = self.scanner.get_next_token();
        if (next_token.get_text() != ")")
        {
            // terminate the program, report invalid input no ;
            eprintln!("Grammatical Error: ParameterBlock: parameter block not ended with ) . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }
        true
    }

    fn FunctionDeclaration(&mut self) -> bool
    {
        if (!self.ParameterBlock())
        {
            eprintln!("Grammatical Error: FunctionDeclaration Function declaration is not a parameter block. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != ";")
        {
            // terminate the program, report invalid input no ;
            eprintln!("Grammatical Error: FunctionDeclaration: Declaratioin not ended with ; . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }
        true
    }

    fn VariableDeclaration(&mut self) -> bool //done
    {
        if (self.scanner.peek_next_token().get_text() == "=")
        {
            let mut next_token = self.scanner.get_next_token();
            if (!self.Constant())
            {
                //report invalid input terminate
                eprintln!("Grammatical Error: FunctionDefinition: = not followed by constant. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                return false;
            }
            else
            {
            }
        }
        else
        {

        }

        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != ";")
        {
            // terminate the program, report invalid input no ;
            eprintln!("Grammatical Error: VariableDeclaration: Declaration not ended with ; . Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }
        true
    }

    fn Block(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.get_next_token();
        if (next_token.get_text() != "{")
        {
            eprintln!("Grammatical Error: Block: Block needs to start with 1. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }

        let mut current_index = self.scanner.get_current_index();
        while (true)
        {
            if (!self.Declaration())
            {
                self.scanner.set_current_index(current_index);
                break;
            }
            current_index = self.scanner.get_current_index();
        }

        current_index = self.scanner.get_current_index();
        while (true)
        {
            if (!self.Statement()) //48
            {
                self.scanner.set_current_index(current_index);
                break;
            }
            current_index = self.scanner.get_current_index();
        }

        current_index = self.scanner.get_current_index();
        while (true)
        {
            if (!self.FunctionDefinition())
            {
                self.scanner.set_current_index(current_index);
                break;
            }
            current_index = self.scanner.get_current_index();
        }

        
        next_token = self.scanner.get_next_token();
        if (next_token.get_text() == "}")
        {
            return true;
        }
        eprintln!("Grammatical Error: Block: Block needs to end with 2. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
        self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
        false
    }

    fn Declaration(&mut self) -> bool //done
    {
        let mut temp_cur_index = self.scanner.get_current_index();
        let mut variable_index = temp_cur_index;
        while (true)
        {
            if (self.scanner.more_tokens_available())
            {
                let mut temp_next_token = self.scanner.get_next_token();
                if (temp_next_token.get_type().as_str() == "Identifier" || temp_next_token.get_type().as_str() == "Function"
                || temp_next_token.get_type().as_str() == "Variable")
                {
                    variable_index = self.scanner.get_current_index() - 1;
                    break;
                }
                
            } 
            else
            {
                break;
            }
        }
        self.scanner.set_current_index(temp_cur_index);

        if (!self.DeclarationType())
        {
            eprintln!("Grammatical Error: Declaration: Missing declaration type for declaration. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }

        eprintln!("nice dec! line {} pos {}",
        self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
        
        let mut current_index = self.scanner.get_current_index();
        if (!self.VariableDeclaration()) 
        {
            eprintln!("failed variable dec");
            self.scanner.set_current_index(current_index);
            if (!self.FunctionDeclaration())
            {
                eprintln!("Grammatical Error: Declaration: Invalid declaration, neither variable declaratioin nor function declaration. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                return false;
            }
            else{
                for i in variable_index..(self.scanner.tokens.len() - 1)
                {
                    if (self.scanner.tokens[i].get_text() == self.scanner.tokens[variable_index].get_text())
                    {
                        self.scanner.tokens[i].change_type(TokenType::FUNCTION);
                    }
                }
            }
            // parallel condition, need to worry about the first function fails the second one because of consuming tokens
        }
        else
        {
            for i in variable_index..(self.scanner.tokens.len() - 1)
            {
                if (self.scanner.tokens[i].get_text() == self.scanner.tokens[variable_index].get_text())
                {
                    self.scanner.tokens[i].change_type(TokenType::VARIABLE);
                }
            }
        }
        
        true
    }

    fn MainDeclaration(&mut self) -> bool //done
    {
        let mut next_token = self.scanner.get_next_token();
        // let mut current_index = self.scanner.get_current_index();
        if (next_token.get_text() == "void")
        {
            next_token = self.scanner.get_next_token();
            if (next_token.get_text() == "main")
            {
                next_token = self.scanner.get_next_token();
                if (next_token.get_text() == "(")
                {
                    next_token = self.scanner.get_next_token();
                    if (next_token.get_text() == ")")
                    {
                        if (self.Block())
                        {
                            return true;
                        }
                        else 
                        {
                            eprintln!("Grammatical Error: MainDeclaration: No block followed after void main(). Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                            process::exit(1);
                            return false;
                        }
                    }
                    else 
                    {
                        eprintln!("Grammatical Error: MainDeclaration: ( need to be followed by ) for main declaration. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                        self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                        process::exit(1);
                        return false;
                    }
                }
                else 
                {
                    eprintln!("Grammatical Error: MainDeclaration: main needs to be followed by () for declaration. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                    self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                    process::exit(1);
                    return false;
                }
            }
            else 
            {
                eprintln!("Grammatical Error: MainDeclaration: Invalid name for main declaration. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                process::exit(1);
                return false;
            }
        }
        else 
        {
            eprintln!("Grammatical Error: MainDeclaration: Main declaration needs to start with void. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            process::exit(1);
            return false;
        }
        
    }

    fn FunctionDefinition(&mut self) -> bool //done
    {
        let mut current_index = self.scanner.get_current_index();
        if (self.DeclarationType())
        {
            current_index = self.scanner.get_current_index();
            if (self.ParameterBlock())
            {
                current_index = self.scanner.get_current_index();
                if (self.Block())
                {
                    return true;
                }
                else
                {
                    // current_index = self.scanner.get_current_index();
                    // eprintln!("Grammatical Error: Parameter block not followed by a block. line {} pos {}",
                    // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                    // process::exit(1);
                    eprintln!("Grammatical Error: FunctionDefinition: No block followed parameter block. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                    self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                    process::exit(1);
                    return false;
                }
            }
            else
            {
                // current_index = self.scanner.get_current_index();
                // eprintln!("Grammatical Error: Declaration type not followed by parameter block. line {} pos {}",
                // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                // process::exit(1);
                eprintln!("Grammatical Error: FunctionDefinition: Function definition not started with DeclarationType. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
                self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
                process::exit(1);
                return false;
            }
        }
        else
        {
            // current_index = self.scanner.get_current_index();
            // eprintln!("Grammatical Error: No declaration type for function definition. line {} pos {}",
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            // process::exit(1);
            // eprintln!("Grammatical Error: FunctionDefinition: Missing declaration type. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            // self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            return false;
        }
    }
    

    fn Program(&mut self) -> bool //done
    {
        let mut current_index = self.scanner.get_current_index();
        while (true) // optional 
        {
            // first declaration part is optional 
            if (!self.Declaration())
            {
                self.scanner.set_current_index(current_index);
                break;
            }
            current_index = self.scanner.get_current_index();
        } 

        current_index = self.scanner.get_current_index();
        if (!self.MainDeclaration())
        {
            // MainDeclaration is required
            // current_index = self.scanner.get_current_index();
            eprintln!("Grammatical Error: Program: No main declaration. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            process::exit(1);
            return false;
        }
        
        if (self.scanner.more_tokens_available())
        {
            current_index = self.scanner.get_current_index();
            while (true)
            {
                if (!self.FunctionDefinition())
                {
                    self.scanner.set_current_index(current_index);
                    break;
                }
                current_index = self.scanner.get_current_index();
            }
        }
        // check again if there still are tokens left
        if (self.scanner.more_tokens_available()) 
        {
            eprintln!("Grammatical Error: Program: More text after program body. Token {} line {} pos {}", self.scanner.peek_next_token().get_text(),
            self.scanner.peek_next_token().get_line_number(), self.scanner.peek_next_token().get_char_pos());
            process::exit(1);
            return false;
        }
        return true;
    }

    pub fn analyze(&mut self)
    {
        if (self.Program())
        {
            println!("it is valid!");
        }else
        {
            println!("it is not valid!");
        }
        
    }

    pub fn write_output(&mut self)
    {
        
		let mut content = String::new();
		let first_line: String = format!(
            "<{}doctype html public {}{}//w3c//dtd xhtml 1.0 transitional//en{} {}http://www.w3.org/tr/xhtml1/dtd/xhtml1-transitional.dtd{}>", '!','"' ,'-' ,'"' ,'"' ,'"');
        // fs::write("output.xhtml", first_line.as_bytes());


        let second_line: String = format!(
			"<html xmlns={}http{}//www.w3.org/1999/xhtml{} xml:lang={}en{}>",
			'"', ':', '"', '"', '"'
		);
        // fs::write("output.xhtml", second_line.as_bytes());
        content.push_str(&first_line);
        content.push('\n');
        content.push_str(&second_line);
        content.push_str("\n");
        content.push_str("<head>\n");
        content.push_str("<title>\n");
		content.push_str("X Formatted file</title>\n</head>\n");
        content.push_str(&format!(
			"<body bgcolor={:?} text={:?} link={:?} vlink={:?}>\n",
			"navy", "yellow", "yellow", "yellow"
		));
        content.push_str(&format!("<font face={:?}>\n", "Courier New"));

        for token in self.scanner.tokens.iter()
        {
            if (token.get_type().as_str() == "Function")
            {
                content.push_str(&format!("<font color={:?}><b>", "orange"));
            }
            else if (token.get_type().as_str() == "Variable")
            {
                content.push_str(&format!("<font color={:?}><b>", "yellow"));
            }
            else if (token.get_type().as_str() == "FloatConstant")
            {
                content.push_str(&format!("<font color={:?}><b>", "aqua"));
            }
            else if (token.get_type().as_str() == "IntConstant")
            {
                content.push_str(&format!("<font color={:?}><b>", "aqua"));
            }
            else if (token.get_type().as_str() == "Operator")
            {
                content.push_str(&format!("<font color={:?}><b>", "white"));
            }   
            else if (token.get_type().as_str() == "Keyword")
            {
                content.push_str(&format!("<font color={:?}><b>", "white"));
            }
            content.push_str(token.get_text());
            content.push_str(&format!("</b></font>"));

           

        
        }
        content.push_str("\n</font>\n</body>\n</html>\n");
		// out_data.push('\n');
		// out_data.push_str(&second_part);
		// out_data.push_str("\n<head>\n");
		// out_data.push_str("<title>\n");
		// out_data.push_str("X Formatted file</title>\n</head>\n");

		// out_data.push_str(&format!(
		// 	"<body bgcolor={:?} text={:?} link={:?} vlink={:?}>\n",
		// 	self.background[0], self.foreground[0], self.foreground[0], self.foreground[0]
		// ));
		// out_data.push_str(&format!("<font face={}{}{}>", '"', self.font[0], '"'));

		// let mut mid_part: String = String::new();
		// let mut last_part: String = String::new();
		// last_part.push_str("\n</font>\n</body>\n</html>\n");
		// out_data.push_str(&last_part);
		fs::write("output.xhtml", content.as_bytes());
    }
    
}


















/*
    when something is optional, the char pos needs to be marked, and if it is not true,
    need to restore the index for accessing tokens 

    find all the parallel or option ones and have them report error at a higher level
*/