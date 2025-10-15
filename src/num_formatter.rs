use crate::math_functions::pow;
use bigdecimal::BigDecimal;
use num_bigint::{Sign, ToBigInt};
use num_traits::Signed;

pub fn num_formatter(num: BigDecimal, significant_figure: u32) -> String {
    if significant_figure == 0 {
        return num.to_string();
    }
    let (a, b) = get_num_of_digit(num.clone());
    if a < significant_figure {
        if a + b < significant_figure {
            num.to_string()
        } else if num < BigDecimal::from(1) {
            let sift_digit = get_num_of_zero(num.clone()) + 1;
            let rsifted =
                num.clone() * pow(BigDecimal::from(10), BigDecimal::from(sift_digit)).unwrap();
            format!(
                "{} * 10^-{}",
                round_n(rsifted, significant_figure - 1),
                sift_digit
            )
            .to_string()
        } else {
            round_n(num, significant_figure - a).to_string()
        }
    } else {
        let rounded = num.round(0) / 10.0_f64.powf((a - significant_figure) as f64);
        let fraction = rounded / 10.0_f64.powf((significant_figure - 1) as f64);
        format!("{} * 10^{}", fraction, a - 1).to_string()
    }
}

pub fn num_hex_formatter(num: BigDecimal, significant_figure: u32) -> String {
    if num.is_integer() {
        let bigint = num.to_bigint().unwrap();
        match bigint.sign() {
            Sign::Minus => format!("-0x{}", bigint.abs().to_str_radix(16)),
            _ => format!("0x{}", bigint.to_str_radix(16)),
        }
    } else {
        num_formatter(num, significant_figure)
    }
}

pub fn num_bin_formatter(num: BigDecimal, significant_figure: u32) -> String {
    if num.is_integer() {
        let bigint = num.to_bigint().unwrap();
        let sign = match bigint.sign() {
            Sign::Minus => "-",
            _ => "",
        };
        let num_bin = bigint.abs().to_str_radix(2);
        let len = ((num_bin.len() - 1) / 8 + 1) * 8;
        let pad_len = len - num_bin.len();
        let mut zeros = "00000000".to_string();
        zeros.truncate(pad_len);
        let pad_added = format!("{zeros}{num_bin}");
        let sep_added = pad_added
            .as_bytes()
            .chunks(8)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join("_");
        format!("{sign}0b{sep_added}")
    } else {
        num_formatter(num, significant_figure)
    }
}

fn get_num_of_digit(num: BigDecimal) -> (u32, u32) {
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

fn get_num_of_zero(num: BigDecimal) -> u32 {
    // num: 0.00012 -> 3
    assert!(num < BigDecimal::from(1));
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

fn round_n(num: BigDecimal, n: u32) -> BigDecimal {
    // num: 123.4567, n: 2 -> 123.45
    (num * pow(BigDecimal::from(10), BigDecimal::from(n)).unwrap()).round(0)
        / pow(BigDecimal::from(10), BigDecimal::from(n)).unwrap()
}

#[cfg(test)]
mod test {
    use super::num_formatter;
    use bigdecimal::{BigDecimal, FromPrimitive};
    use std::io::Write;
    use text_colorizer::*;

    struct TestCase {
        num: BigDecimal,
        sf: u32,
        result: String,
    }

    #[test]
    fn test_calc() {
        let test_cases = get_testcases();
        let mut test_success = 0;
        for (i, tc) in test_cases.iter().enumerate() {
            let res = num_formatter(tc.num.clone(), tc.sf);
            if res == tc.result {
                writeln!(
                    &mut std::io::stderr(),
                    "testcase {}: {} '{}, {} -> {}'",
                    i,
                    "SUCCESSED          ".green(),
                    tc.num,
                    tc.sf,
                    tc.result,
                )
                .unwrap();
                test_success += 1;
            } else {
                writeln!(
                    &mut std::io::stderr(),
                    "testcase {}: {} '{}, {} -> {}', but expected {}",
                    i,
                    "FORMAT FAILED ".red(),
                    tc.num,
                    tc.sf,
                    res,
                    tc.result,
                )
                .unwrap();
            }
        }
        writeln!(
            &mut std::io::stderr(),
            "testcase {}/{} SUCCESSED",
            test_success,
            test_cases.len(),
        )
        .unwrap();
        assert_eq!(test_success, test_cases.len());
    }

    fn get_testcases() -> Vec<TestCase> {
        let mut test_cases: Vec<TestCase> = Vec::new();
        test_cases.push(TestCase {
            num: BigDecimal::from_f64(12.3456789).unwrap(),
            sf: 4,
            result: "12.35".to_string(),
        });
        test_cases
    }
}
