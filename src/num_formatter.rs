pub fn num_formatter(num: f64, significant_figure: u32) -> String {
    if significant_figure == 0 {
        return num.to_string();
    }
    let (a, b) = get_num_of_digit(num);
    if a < significant_figure {
        if a + b < significant_figure {
            num.to_string()
        } else if num < 1.0 {
            let sift_digit = get_num_of_zero(num) + 1;
            let rsifted = num * 10_f64.powf(sift_digit as f64) as f64;
            format!(
                "{} * 10^-{}",
                round_n(rsifted, (significant_figure - 1) as f64),
                sift_digit
            )
            .to_string()
        } else {
            round_n(num, (significant_figure - a) as f64).to_string()
        }
    } else {
        let rounded = num.round() / 10.0_f64.powf((a - significant_figure) as f64);
        let fraction = rounded / 10.0_f64.powf((significant_figure - 1) as f64);
        format!("{} * 10^{}", fraction, a - 1).to_string()
    }
}

fn get_num_of_digit(num: f64) -> (u32, u32) {
    // num: 3.14 -> (1, 2)
    enum State {
        Seisu,
        Syosu,
    }
    let mut state = State::Seisu;
    let mut a = 0;
    let mut b = 0;
    for c in num.to_string().chars() {
        if c != '.' {
            match state {
                State::Seisu => a += 1,
                State::Syosu => b += 1,
            }
        } else {
            state = State::Syosu;
        }
    }
    (a, b)
}

fn get_num_of_zero(num: f64) -> u32 {
    // num: 0.00012 -> 3
    assert!(num < 1.0);
    let mut num_of_zero = 0;
    for c in num.to_string().replace("0.", "").chars() {
        if c == '0' {
            num_of_zero += 1;
        } else {
            break;
        }
    }
    num_of_zero
}

fn round_n(num: f64, n: f64) -> f64 {
    // num: 123.4567, n: 2 -> 123.45
    (num * 10.0_f64.powf(n)).round() / 10.0_f64.powf(n)
}
