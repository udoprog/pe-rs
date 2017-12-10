fn run(end: u32) -> u32 {
    // starts on a monday.
    let mut days = (0..7).cycle();
    days.next();

    return (1900..end)
        .flat_map(|year| calendar_days(is_leap(year)))
        .zip(days)
        .filter(|&(cal, day)| day % 7 == 0 && cal == 0)
        .count() as u32;

    fn is_leap(year: u32) -> bool {
        match year {
            year if year % 100 != 0 && year % 4 == 0 => true,
            year if year % 100 == 0 && year % 400 == 0 => true,
            _ => false,
        }
    }

    /// Construct iterator for calendar days for all months in a year.
    fn calendar_days(leap: bool) -> impl Iterator<Item = u32> {
        (0..12).flat_map(move |m| dom(leap, m))
    }

    /// Construct iterator for days in a given months.
    fn dom(leap: bool, month: u32) -> impl Iterator<Item = u32> {
        let len = match month {
            0 => 30, // jan
            1 => if leap { 28 } else { 29 }, // feb
            2 => 31, // mar
            3 => 30, // apr
            4 => 31, // may
            5 => 30, // jun
            6 => 31, // jul
            7 => 31, // aug
            8 => 30, // sep
            9 => 31, // oct
            10 => 30, // nov
            11 => 31, // dec
            m => panic!("bad month: {}", m),
        };

        0..len
    }
}

problem!{
    tests => [
        example1 => (run(1901), 2),
        q => {run(2000), "284de502c9847342318c17d474733ef468fbdbe252cddf6e4b4be0676706d9d0"},
    ];
}
