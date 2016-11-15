use std::io::{BufReader, Bytes, Read};
use std::str::FromStr;


pub struct NumberScanner<R: Read> {
    bytes: Bytes<BufReader<R>>,
}

impl<R: Read> NumberScanner<R> {
    pub fn new(br: BufReader<R>) -> Self {
        NumberScanner { bytes: br.bytes() }
    }

    fn next_token(&mut self) -> Option<String> {
        let mut token = String::new();
        loop {
            if let Some(Ok(b)) = self.bytes.next() {
                if b == b' ' || b == b'\n' {
                    if token.len() > 0 {
                        break;
                    } else {
                        continue;
                    }
                }

                token.push(b as char);
            } else {
                break;
            }
        }

        if token.len() > 0 { Some(token) } else { None }
    }

    pub fn next<F: FromStr>(&mut self) -> Option<F> {
        self.next_token().and_then(|token| {
            if let Ok(x) = token.parse::<F>() {
                Some(x)
            } else {
                None
            }
        })
    }
}
