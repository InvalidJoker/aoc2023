pub fn get_digits(i: &str) -> [u32; 2] {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first = None;
    let mut last = 0;

    let mut digit = |c| {
        first = first.or(Some(c));
        last = c;
    };

    let chars = i.as_bytes();
    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            digit((c - b'0') as u32);
        } else {
            for (j, d) in digits.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    digit(j as u32 + 1);
                }
            }
        }
    }

    [first.unwrap(), last]
}
