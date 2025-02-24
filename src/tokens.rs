#[derive(Debug, Clone,PartialEq)]
pub struct Word {
    pub letters: Vec<char>,
}

impl Word {
    #[inline]
    pub fn new() -> Self {
        Word { letters: Vec::with_capacity(16) }
    }
    #[inline]
    pub fn clear(&mut self) {
        self.letters.clear();
    }
}

#[derive(Debug, Clone,PartialEq)]
pub enum Token {
    FnK,
    EqSign,
    Iden(Word),
    LSmallB,
    RSmallB,
    LCurlyB,
    RCurlyB,
    EOL,
    EOF,
    AtTheRate,
    EMP,
    SemiC,
    Comma,
    Quote,
}

impl Token {
    pub fn new() -> Self {
        Self::EMP
    }
    pub fn run_lexical_analyzer(code: &str) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(code.len() / 4);
        let mut current_word = Word::new();
        let mut chars = code.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::Quote);
                }
                ',' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::Comma);
                }
                ';' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::SemiC);
                }
                '@' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::AtTheRate);
                }
                '=' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::EqSign);
                }
                '\n' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::EOL);
                }
                ' ' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                }
                '(' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::LSmallB);
                }
                ')' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::RSmallB);
                }
                '{' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::LCurlyB);
                }
                '}' => {
                    Self::process_word(&mut tokens, &current_word);
                    current_word.clear();
                    tokens.push(Token::RCurlyB);
                }
                _ => {
                    current_word.letters.push(c);
                }
            }
        }
        Self::process_word(&mut tokens, &current_word);
        tokens.push(Token::EOF);

        tokens
    }

    #[inline(always)]
    fn process_word(tokens: &mut Vec<Token>, word: &Word) {
        if word.letters.is_empty() {
            return;
        }
        if word.letters.as_slice() == ['f', 'u', 'n'] {
            tokens.push(Token::FnK);
        } else {
            tokens.push(Token::Iden(word.clone()));
        }
    }
}