use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
enum Tokens {
    PLUS,
    MINUS,
    MUL,
    DIV,
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
    WORD(String),
    INT(i32),
}

#[derive(Clone)]
struct Lexer {
    pub token_stack: VecDeque<Tokens>,
} 

struct Interpreter {
    stack: Vec<i32>,
    word_map: std::collections::HashMap<String, VecDeque<Tokens>>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            token_stack: VecDeque::new(),
        }
    }

    fn read_file(&mut self, filename: &str) -> String {
        let raw_data = std::fs::read_to_string(filename).unwrap();
        raw_data
    }

    fn push(&mut self, value: Tokens) {
        self.token_stack.push_back(value);
    }
}

macro_rules! binexpr {
    ($interpreter:ident, $symbol:tt) => {
        let a = $interpreter.pop();
        let b = $interpreter.pop();
        $interpreter.push(b $symbol a);
    }
}

impl Interpreter {
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
                    Tokens::DOT => println!("{}", self.pop()),
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
                    Tokens::SEMI => {
                    },
                    Tokens::EMIT => print!("{}", (self.pop() as u8) as char),
                    Tokens::CR => println!(""),
                    Tokens::DROP => {self.pop();},
                    Tokens::DUP => self.dup(),
                    Tokens::SWAP => self.swap(),
                    Tokens::OVER => self.over(),
                    Tokens::ROT => self.rot(),
                    Tokens::WORD(word) => {
                        match self.word_map.get(&word) {
                            Some(tokens) => self.interpret_tokens(tokens.clone()),
                            None => panic!("undefined word {}", &word),
                        }
                    },
                    Tokens::INT(num) => self.push(num),
                }
            };
        }
    }
}

fn get_token_from_word(word: &str) -> Tokens {
    match word {
        "+" => return Tokens::PLUS,
        "-" => return Tokens::MINUS,
        "*" => return Tokens::MUL,
        "/" => return Tokens::DIV,
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
        _ => match word.parse::<i32>() {
            Ok(num) => return Tokens::INT(num),
            Err(_) => return Tokens::WORD(word.to_string()),
        }
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough args");
    }
    let mut lexer = Lexer::new();
    let raw = lexer.read_file(&args[1]);
    let data = raw.split_whitespace();

    for word in data {
        let token = get_token_from_word(word);
        lexer.push(token);
    }

    let mut interpreter = Interpreter{
        stack: vec![], 
        word_map: std::collections::HashMap::new(),
    };

    interpreter.interpret_tokens(lexer.token_stack);

}
