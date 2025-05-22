use crate::cfg::instr::{Instr};

pub struct Parser<'s> {
    // Tokenises the source code
    lexer: Lexer<'s>,

}

pub fn is_keyword(s: &str) ->bool {
    matches!(
        s,
        "return",
    )
}

impl<'s, 't> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            lexer: Lexer {
                source,
                remaining: source,
            }
        }

        }

    fn matchx(&mut self, prefix: &str) ->bool {
        self.lexer.matchx(prefix)
    }

    pub fn parse(&mut self) {
        // Parse whole program
        // Only return for now
        // add it to current BB
        let ret: Option<Instr> = self.parse_statement();
    }

    pub fn parse_expression(&mut self) -> i32 {
        return 10;
    }

    pub fn parse_statement(&mut self) -> Option<Instr> {
        if self.matchx("return") {

            Some(Instr {
                opcode: crate::cfg::instr::Opcode::Return,
                ..Default::default()
            })
        } else {
            None
        }
    }

}
struct Lexer<'a> {
    // The entire output
    source: &'a str,
    remaining: &'a str,
}

fn is_number(c: char) ->bool {
    c.is_ascii_digit()
}

// includes numbers as well, but id can't start with an umber
fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

// includes numbers as well, but id can't start with an umber
fn is_id_letter(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_punctuation(c: char) -> bool{
    "=;[]<>()+-/*&|^".contains(c)
}

impl<'a> Lexer<'a> {
    fn is_eof(&self) -> bool {
        self.remaining.is_empty()
    }


    fn matchx(&mut self, syntax: &str) -> bool {
            self.skip_whitespace();

            let mut input_chars = self.remaining.chars();
            let mut temp_len = 0;

            for expected_char in syntax.chars() {
                match input_chars.next() {
                    Some(c) if c == expected_char => {
                        temp_len += c.len_utf8();
                    }
                    _ => return false,
                }
            }

            self.remaining = &self.remaining[temp_len..];
            true
    }
    fn peek(&self) -> Option<char> {
        self.remaining.chars().next()
    }

    // advances the state of remaining by skipping over the next character
    fn next_char(&mut self) -> Option<char> {
        self.peek()
            .inspect(|c| self.remaining = &self.remaining[c.len_utf8()..])
    }

    fn is_whitespace(&self) -> bool {
        self.peek().as_ref().is_some_and(char::is_ascii_whitespace)
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_whitespace() {
                self.next_char();
            } else if self.remaining.starts_with("//") {
                if let Some(i) = self.remaining.find('\n') {
                    self.remaining = &self.remaining[i + 1..];
                } else {
                    self.remaining = "";
                }
            } else {
                // the parser now is positioned at something meaningful
                break;
            }
        }
    }


    fn integer(&mut self) -> (bool, &'a str) {
        let old = self.remaining;

        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.next_char();
        }

        if self.remaining == old {
            return (false, "");
        }

        (true, &old[..old.len() - self.remaining.len()])
    }

}