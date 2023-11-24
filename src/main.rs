

#[derive(Debug)]
enum Tokens {
    PLUS,
    MINUS,
    MUL,
    DIV,
    DOT,
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

struct Lexer {
    pub token_stack: Vec<Tokens>,
} 

struct Interpreter {
    stack: Vec<i32>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            token_stack: vec![],
        }
    }

    fn read_file(&mut self, filename: &str) -> String {
        let raw_data = std::fs::read_to_string(filename).unwrap();
        raw_data
    }

    fn push(&mut self, value: Tokens) {
        self.token_stack.push(value);
    }
}

impl Interpreter {
    fn push(&mut self, value: i32){
        self.stack.push(value);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
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
}

fn get_token_from_word(word: &str) -> Tokens {
    match word {
        "+" => return Tokens::PLUS,
        "-" => return Tokens::MINUS,
        "*" => return Tokens::MUL,
        "/" => return Tokens::DIV,
        "." => return Tokens::DOT,
        "emit" => return Tokens::EMIT,
        "cr" => return Tokens::CR,
        "drop" => return Tokens::DROP,
        "dup" => return Tokens::DUP,
        "swap" => return Tokens::SWAP,
        "over" => return Tokens::OVER,
        "rot" => return Tokens::ROT,
        _ => if word.parse::<i32>().is_ok() {
            return Tokens::INT(word.parse::<i32>().unwrap());
        } else {
            return Tokens::WORD(word.to_string());
        }
    }
}

macro_rules! binexpr {
    ($interpreter:ident, $symbol:tt) => {
        let a = $interpreter.pop();
        let b = $interpreter.pop();
        $interpreter.push(b $symbol a);
    }
}

fn main() {
    let mut lexer = Lexer::new();
    let raw = lexer.read_file("test.forth");
    let data = raw.split_whitespace();

    for word in data {
        let token = get_token_from_word(word);
        lexer.push(token);
    }

    let mut interpreter = Interpreter{stack: vec![]};

    for token in lexer.token_stack.iter() {
        match &token {
            Tokens::PLUS => {binexpr!(interpreter, +);},
            Tokens::MINUS => {binexpr!(interpreter, -);},
            Tokens::MUL => {binexpr!(interpreter, *);},
            Tokens::DIV => {binexpr!(interpreter, /);},
            Tokens::DOT => println!("{}", interpreter.pop()),
            Tokens::EMIT => print!("{}", (interpreter.pop() as u8) as char),
            Tokens::CR => println!(""),
            Tokens::DROP => {interpreter.pop();},
            Tokens::DUP => interpreter.dup(),
            Tokens::SWAP => interpreter.swap(),
            Tokens::OVER => interpreter.over(),
            Tokens::ROT => interpreter.rot(),
            Tokens::WORD(word) => println!("{}", word),
            Tokens::INT(num) => interpreter.push(*num),
        }
    }


}
