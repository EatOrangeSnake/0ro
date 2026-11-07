use std::io::{Read, Error};

pub struct Tree {
    pub concepts: String
}

pub struct TreeFactory<'a, R: Read> {
    pub source: R, 
    pub buf: &'a mut [u8], 
    pub tree: Tree, 
    pub current: String
}

impl<'a, R: Read> Iterator for TreeFactory<'a, R> {
    type Item = Result<(), Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.source.read(self.buf).ok()?;
        let mut last: usize = 0;
        let s = str::from_utf8(self.buf).ok()?;
        for (p, c) in s.char_indices() {
            match c {
                ' ' => {
                    if last == p {
                        last += 1;
                    } else {
                        last = p;
                    }
                }
                _ => {}
            }
        }
        self.current.clear();
        self.current.push_str(&s[last..]);
        Some(Ok(()))
    }
}
