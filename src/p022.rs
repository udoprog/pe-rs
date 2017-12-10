use std::io::{self, Cursor, Read, BufReader};

struct Names<R>
where
    R: Read,
{
    chars: io::Chars<BufReader<R>>,
    buffer: String,
}

impl<R> Names<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Names<R> {
        Names {
            chars: BufReader::new(reader).chars(),
            buffer: String::new(),
        }
    }
}

impl<R> Iterator for Names<R>
where
    R: Read,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut parens = false;

        while let Some(c) = self.chars.next() {
            let c = c.expect("read error");

            match c {
                '"' if !parens => {
                    parens = true;
                }
                '"' if parens => {
                    let buf = self.buffer.clone();
                    self.buffer.clear();
                    return Some(buf);
                }
                c if parens => {
                    self.buffer.push(c);
                }
                ',' => {}
                _ => panic!("bad character: {}", c),
            }
        }

        None
    }
}

fn run<R>(reader: R) -> u64
where
    R: Read,
{
    let mut sum = 0u64;

    let mut names: Vec<String> = Names::new(reader).collect();
    names.sort();

    for (i, name) in names.into_iter().enumerate() {
        let s: u64 = name.chars().map(|c| c as u64 - 'A' as u64 + 1).sum::<u64>() * (i as u64 + 1);
        sum += s;
    }

    sum
}

const INPUT: &str = include_str!("p022_names.txt");

problem!{
    tests => [
        q => {run(Cursor::new(INPUT)), "85148c096c25e3ed3da55c7e9c89448018b0f5f53ad8d042129c33d9beac6736"},
    ];
}
