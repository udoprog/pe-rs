use std::collections::VecDeque;

fn size(input: u32) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(input);

    let mut out = Vec::new();

    while let Some(input) = queue.pop_front() {
        let o = match input {
            0 => continue,
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            v if v / 1000 >= 1 => {
                queue.push_back(input % 1000);
                queue.push_back(input / 1000);
                "thousand"
            }
            v if v / 100 >= 1 => {
                queue.push_back(input / 100);

                if input % 100 != 0 {
                    out.push("and");
                    queue.push_back(input % 100);
                }

                "hundred"
            }
            v if v / 10 >= 1 => {
                queue.push_back(input % 10);

                match v / 10 {
                    2 => "twenty",
                    3 => "thirty",
                    4 => "forty",
                    5 => "fifty",
                    6 => "sixty",
                    7 => "seventy",
                    8 => "eighty",
                    9 => "ninety",
                    _ => panic!("can't handle: {}", input),
                }
            }
            _ => panic!("can't handle: {}", input),
        };

        out.push(o);
    }

    out.into_iter().map(|c| c.len()).sum()
}

fn run(limit: u32) -> u64 {
    (1..=limit).map(|n| size(n) as u64).sum()
}

problem!{
    tests => [
        example1 => (run(5), 19),
        q => {run(1000), "1a455b216c6e916943acf3fa4c7e57a7a5cac66d97cc51befca810c223ef9c23"},
        size1 => (size(43), "fortythree".len()),
        size2 => (size(342), "threehundredandfortytwo".len()),
        size4 => (size(1000), "onethousand".len()),
        size5 => (size(26), "twentysix".len()),
        size6 => (size(999), "ninehundredandninetynine".len()),
        size7 => (size(778), "sevenhundredandseventyeight".len()),
        size8 => (size(115), 20),
    ];
}
