use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
enum Tokens {
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    DOT,
    COLON,
    SEMI,
    EMIT,
    CR,
    DROP,
    DUP,
    SWAP,
    OVER,
    ROT,
    AND,
    OR,
    INVERT,
    DOTQUOTE,
    QUOTE,
    LESS,
    GREATER,
    EQ,
    IF,
    ELSE,
    THEN,
    DO,
    LOOP,
    BEGIN,
    UNTIL,
    I,
    VARIABLE,
    CONSTANT,
    AT,
    QUESTION,
    BANG,
    BANGITER,
    ALLOT,
    CELLS,
    KEY,
    STRING(String),
    WORD(String),
    INT(i32),
}

const CELL_WIDTH: u8 = 1;

#[derive(Clone)]
struct Lexer {
    pub token_stack: VecDeque<Tokens>,
} 

struct Loop {
    cur_iter: i32,
    in_loop: bool,
}

struct Interpreter {
    stack: Vec<i32>,
    if_stack: Vec<bool>,
    memory_stack: Vec<i32>,
    memory_map: std::collections::HashMap<String, usize>,
    constant_map: std::collections::HashMap<String, i32>,
    word_map: std::collections::HashMap<String, VecDeque<Tokens>>,
    in_word: bool,
    cur_loop: Loop,
    loop_tokens: VecDeque<Tokens>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            token_stack: VecDeque::new(),
        }
    }

    fn get_token_from_word(&self, word: &str) -> Tokens {
        match word {
            "+" => return Tokens::PLUS,
            "-" => return Tokens::MINUS,
            "*" => return Tokens::MUL,
            "/" => return Tokens::DIV,
            "mod" => return Tokens::MOD,
            "." => return Tokens::DOT,
            ":" => return Tokens::COLON,
            ";" => return Tokens::SEMI,
            "emit" => return Tokens::EMIT,
            "cr" => return Tokens::CR,
            "drop" => return Tokens::DROP,
            "dup" => return Tokens::DUP,
            "swap" => return Tokens::SWAP,
            "over" => return Tokens::OVER,
            "rot" => return Tokens::ROT,
            "and" => return Tokens::AND,
            "or" => return Tokens::OR,
            "invert" => return Tokens::INVERT,
            ".\"" => return Tokens::DOTQUOTE,
            "\"" => return Tokens::QUOTE,
            "<" => return Tokens::LESS,
            ">" => return Tokens::GREATER,
            "=" => return Tokens::EQ,
            "if" => return Tokens::IF,
            "else" => return Tokens::ELSE,
            "then" => return Tokens::THEN,
            "do" => return Tokens::DO,
            "loop" => return Tokens::LOOP,
            "begin" => return Tokens::BEGIN,
            "until" => return Tokens::UNTIL,
            "i" => return Tokens::I,
            "variable" => return Tokens::VARIABLE,
            "constant" => return Tokens::CONSTANT,
            "@" => return Tokens::AT,
            "?" => return Tokens::QUESTION,
            "!" => return Tokens::BANG,
            "+!" => return Tokens::BANGITER,
            "allot" => return Tokens::ALLOT,
            "cells" => return Tokens::CELLS,
            "key" => return Tokens::KEY,
            _ => match word.parse::<i32>() {
                Ok(num) => return Tokens::INT(num),
                Err(_) => return Tokens::WORD(word.to_string()),
            }
        }
    }

    fn generate_tokens(&mut self, filename: &str) {
        let raw_data = std::fs::read_to_string(filename).expect("Failed to read file");
        let data: VecDeque<&str> = raw_data.split_whitespace().collect();
        self.lex_data(data);
    }

    fn push(&mut self, value: Tokens) {
        self.token_stack.push_back(value);
    }

    fn lex_data(&mut self, mut data: VecDeque<&str>){
        while data.len() > 0 {
            let mut word: String = Default::default();
            match data.pop_front() {
                Some(some_word) => {word = some_word.to_string();},
                None => panic!("Unknown keyword {}", word),
            }
            let token = self.get_token_from_word(&word);
            match token {
                Tokens::DOTQUOTE => {
                    let str: Vec<&str> = data.clone().into_iter().take_while(|&v| self.get_token_from_word(v) != Tokens::QUOTE).collect(); 
                    data = data.split_off(str.len());
                    self.push(Tokens::STRING(str.join(" ")));
                },
                _ => self.push(token),
            }
        }
    }
}

macro_rules! binexpr {
    ($interpreter:ident, $symbol:tt) => {
        let a = $interpreter.pop();
        let b = $interpreter.pop();
        $interpreter.push((b $symbol a) as i32);
    }
}

macro_rules! compexpr {
    ($interpreter:ident, $symbol:tt) => {
        let a = $interpreter.pop();
        let b = $interpreter.pop();
        $interpreter.push((b $symbol a) as i32 * -1);
    }
}



impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(), 
            if_stack: Vec::new(), 
            memory_stack: Vec::new(), 
            memory_map: std::collections::HashMap::new(),
            constant_map: std::collections::HashMap::new(),
            word_map: std::collections::HashMap::new(),
            in_word: false,
            cur_loop: Loop{
                cur_iter: 0,
                in_loop: false,
            },
            loop_tokens: VecDeque::new(),
        }
    }

    fn push(&mut self, value: i32){
        self.stack.push(value);
    }

    fn pop(&mut self) -> i32 {
        match self.stack.pop() {
            Some(num) => num,
            None => panic!("empty stack"),
        }
    } 

    fn dup(&mut self) {
        let a = self.pop();
        self.push(a);
        self.push(a);
    }

    fn swap(&mut self) {
        let a = self.pop();
        let b = self.pop();
        self.push(a);
        self.push(b);
    }

    fn over(&mut self) {
        let a = self.pop();
        let b = self.pop();
        self.push(b);
        self.push(a);
        self.push(b);
    }
    
    fn rot(&mut self) {
        let a = self.pop();
        let b = self.pop();
        let c = self.pop();
        self.push(b);
        self.push(a);
        self.push(c);
    }

    fn invert(&mut self) {
        let a = self.pop();
        self.push(-a - 1);
    }

    fn handle_word(&mut self, word: &String){
        match self.word_map.get(word) {
            Some(tokens) => { 
                self.in_word = true;
                self.interpret_tokens(tokens.clone());
            },
            None => {
                match self.memory_map.get(word) {
                    Some(index) => self.push(*index as i32),
                    None => {
                        match self.constant_map.get(word) {
                            Some(value) => self.push(*value),
                            None => panic!("undefined word {}", word),
                        }
                    }
                }
            }
        }
    }

    fn add_word(&mut self, word: String, tokens: VecDeque<Tokens>) {
        self.word_map.insert(word, tokens);
    }

    fn interpret_tokens(&mut self, mut tokens: VecDeque<Tokens>) {
        while tokens.len() > 0 {
            if let Some(token) = tokens.pop_front() {
                match token {
                    Tokens::PLUS => {binexpr!(self, +);},
                    Tokens::MINUS => {binexpr!(self, -);},
                    Tokens::MUL => {binexpr!(self, *);},
                    Tokens::DIV => {binexpr!(self, /);},
                    Tokens::MOD => {binexpr!(self, %);},
                    Tokens::DOT => print!("{}", self.pop()),
                    Tokens::COLON => {
                        let word_token = tokens.pop_front();
                        let current_word: String;
                        match word_token {
                            Some(Tokens::WORD(name)) => {current_word = name.to_string();},
                            _ => panic!("expected word"),
                        }
                        let token_vec: VecDeque<Tokens> = tokens.clone().into_iter().take_while(|v| *v != Tokens::SEMI).collect();
                        tokens = tokens.split_off(token_vec.len());
                        self.add_word(current_word, token_vec);
                    },
                    Tokens::SEMI => {self.in_word = false;},
                    Tokens::EMIT => print!("{}", (self.pop() as u8) as char),
                    Tokens::CR => println!(""),
                    Tokens::DROP => {self.pop();},
                    Tokens::DUP => self.dup(),
                    Tokens::SWAP => self.swap(),
                    Tokens::OVER => self.over(),
                    Tokens::ROT => self.rot(),
                    Tokens::AND => {binexpr!(self, &);},
                    Tokens::OR => {binexpr!(self, |);},
                    Tokens::INVERT => {self.invert()},
                    Tokens::DOTQUOTE => {
                        let str: Vec<Tokens> = tokens.clone().into_iter().take_while(|v| *v != Tokens::QUOTE).collect();
                        println!("{:?}", str);
                    },
                    Tokens::QUOTE => (),
                    Tokens::LESS => {compexpr!(self, <);},
                    Tokens::GREATER => {compexpr!(self, >);},
                    Tokens::EQ => {compexpr!(self, ==);},
                    Tokens::IF => {
                        if !self.in_word {
                            panic!("expected to be in word");
                        }
                        let a = self.pop();
                        self.if_stack.push(a != 0);
                        if a == 0 {
                            let token_vec: VecDeque<Tokens> = tokens.clone().into_iter().take_while(|v| (*v != Tokens::THEN) && (*v != Tokens::ELSE)).collect();
                            tokens = tokens.split_off(token_vec.len() + 1);
                        } else {
                            continue;
                        }
                    },
                    Tokens::ELSE => {
                        if !self.in_word {
                            panic!("expected to be in word");
                        }
                        let a = self.if_stack.pop().expect("Stack underflow");
                        if a {
                            let token_vec: VecDeque<Tokens> = tokens.clone().into_iter().take_while(|v| *v != Tokens::THEN).collect();
                            tokens = tokens.split_off(token_vec.len() + 1);
                        } else {
                            continue;
                        }
                    },
                    Tokens::THEN => {},
                    Tokens::DO => {
                        let a = self.pop();
                        self.cur_loop.in_loop = true;
                        self.cur_loop.cur_iter = a;
                        let b = self.pop();
                        let token_vec: VecDeque<Tokens> = tokens.clone().into_iter().take_while(|v| *v != Tokens::LOOP).collect();
                        tokens = tokens.split_off(token_vec.len());
                        while self.cur_loop.cur_iter < b {
                            self.interpret_tokens(token_vec.clone());
                            self.cur_loop.cur_iter += 1;
                        }
                    },
                    Tokens::LOOP => {self.cur_loop.in_loop = false;},
                    Tokens::BEGIN => {
                        self.loop_tokens = tokens.clone().into_iter().take_while(|v| *v != Tokens::UNTIL).collect();
                        self.loop_tokens.push_back(Tokens::UNTIL);
                    },
                    Tokens::UNTIL => {if self.pop() == 0 {self.interpret_tokens(self.loop_tokens.clone())}},
                    Tokens::I => {if self.cur_loop.in_loop { self.push(self.cur_loop.cur_iter);} else {panic!("error: i cannot be outside loop");}},
                    Tokens::VARIABLE => {
                        match tokens.pop_front() {
                            Some(Tokens::WORD(name)) => {
                                self.memory_map.insert(name, self.memory_stack.len());
                                self.memory_stack.push(0);
                            },
                            _ => panic!("unexpected keyword"),
                        }
                    },
                    Tokens::CONSTANT => {
                        let a = tokens.pop_front().expect("Stack underflow");
                        let b = self.pop();
                        match a {
                            Tokens::WORD(name) => self.constant_map.insert(name, b),
                            _ => panic!("error: unexpected keyword"),
                        };
                    },
                    Tokens::AT => {
                        let a = self.pop();
                        self.push(self.memory_stack[a as usize]);
                    },
                    Tokens::QUESTION => {
                        let a = self.pop();
                        print!("{}", self.memory_stack[a as usize]);
                    },
                    Tokens::BANG => {
                        let a = self.pop();
                        let b = self.pop();
                        self.memory_stack[a as usize] = b;
                    },
                    Tokens::BANGITER => {
                        let a = self.pop();
                        self.memory_stack[a as usize] = self.memory_stack[a as usize] + 1;
                    },
                    Tokens::ALLOT => {
                        let a = self.pop();
                        self.memory_stack.append(&mut vec![0; a as usize]);
                    },
                    Tokens::CELLS => {
                        let a = self.pop();
                        self.push(a * CELL_WIDTH as i32);
                    },
                    Tokens::KEY => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("could not read line");
                        let key = input.as_bytes()[0] as i32;
                        self.push(key);
                    },
                    Tokens::STRING(str) => print!("{}", str),
                    Tokens::WORD(word) => {self.handle_word(&word);},
                    Tokens::INT(num) => self.push(num),
                }
            };
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program_name = &args[0];
    if args.len() < 2 {
        panic!("usage: {} <filename.forth>", program_name);
    }
    let file_name = &args[1];

    let mut lexer = Lexer::new();
    lexer.generate_tokens(file_name);

    let mut interpreter = Interpreter::new();

    interpreter.interpret_tokens(lexer.token_stack);

}
