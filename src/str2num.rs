use crate::error::MyError;
use crate::tokenizer::NumFormat;
use bigdecimal::BigDecimal;
use num_traits::ToPrimitive;
use std::str::FromStr;

fn hex2dec_f64(num_str: &str) -> Result<f64, MyError> {
    let mut num: f64 = 0.0;
    let mut figure: f64 = 1.0;
    for i in num_str.chars() {
        match f64::from_str(&i.to_string()) {
            Ok(n) => {
                num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                figure += 1.0;
            }
            Err(_) => {
                let n: f64 = match &i.to_string()[0..1] {
                    "a" | "A" => 10.0,
                    "b" | "B" => 11.0,
                    "c" | "C" => 12.0,
                    "d" | "D" => 13.0,
                    "e" | "E" => 14.0,
                    "f" | "F" => 15.0,
                    _ => return Err(MyError::InvalidHexFormat(num_str.to_string())),
                };
                num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                figure += 1.0;
            }
        }
    }
    Ok(num)
}

fn hex2dec_u64(num_str: &str) -> Result<u64, MyError> {
    let mut num: u64 = 0;
    let mut figure: u32 = 1;
    for i in num_str.chars() {
        match u64::from_str(&i.to_string()) {
            Ok(n) => {
                num += n * 16_u64.pow(num_str.len() as u32 - figure);
                figure += 1;
            }
            Err(_) => {
                let n: u64 = match &i.to_string()[0..1] {
                    "a" | "A" => 10,
                    "b" | "B" => 11,
                    "c" | "C" => 12,
                    "d" | "D" => 13,
                    "e" | "E" => 14,
                    "f" | "F" => 15,
                    _ => return Err(MyError::InvalidHexFormat(num_str.to_string())),
                };
                num += n * 16_u64.pow(num_str.len() as u32 - figure);
                figure += 1;
            }
        }
    }
    Ok(num)
}

fn bin2dec_f64(num_str: &str) -> Result<f64, MyError> {
    let mut num: f64 = 0.0;
    let mut figure: f64 = 1.0;
    for i in num_str.chars() {
        match f64::from_str(&i.to_string()) {
            Ok(n) => {
                if n > 1.0_f64 {
                    return Err(MyError::InvalidBinFormat(num_str.to_string()));
                }
                num += n * 2.0_f64.powf(num_str.len() as f64 - figure);
                figure += 1.0;
            }
            Err(e) => return Err(MyError::ParseFloatError(e)),
        }
    }
    Ok(num)
}

fn bin2dec_u64(num_str: &str) -> Result<u64, MyError> {
    let mut num: u64 = 0;
    let mut figure: u32 = 1;
    for i in num_str.chars() {
        match u64::from_str(&i.to_string()) {
            Ok(n) => {
                if n > 1 {
                    return Err(MyError::InvalidBinFormat(num_str.to_string()));
                }
                num += n * 2_u64.pow(num_str.len() as u32 - figure);
                figure += 1;
            }
            Err(e) => return Err(MyError::ParseIntError(e)),
        }
    }
    Ok(num)
}

fn oct2dec_f64(num_str: &str) -> Result<f64, MyError> {
    let mut num: f64 = 0.0;
    let mut figure: f64 = 1.0;
    for i in num_str.chars() {
        match f64::from_str(&i.to_string()) {
            Ok(n) => {
                if n > 7.0_f64 {
                    return Err(MyError::InvalidOctalFormat(num_str.to_string()));
                }
                num += n * 8.0_f64.powf(num_str.len() as f64 - figure);
                figure += 1.0;
            }
            Err(e) => return Err(MyError::ParseFloatError(e)),
        }
    }
    Ok(num)
}

fn oct2dec_u64(num_str: &str) -> Result<u64, MyError> {
    let mut num: u64 = 0;
    let mut figure: u32 = 1;
    for i in num_str.chars() {
        match u64::from_str(&i.to_string()) {
            Ok(n) => {
                if n > 7 {
                    return Err(MyError::InvalidOctalFormat(num_str.to_string()));
                }
                num += n * 8_u64.pow(num_str.len() as u32 - figure);
                figure += 1;
            }
            Err(e) => return Err(MyError::ParseIntError(e)),
        }
    }
    Ok(num)
}

fn scientific2dec_f64(num_str: &str) -> Result<f64, MyError> {
    let bigdecimal = match BigDecimal::from_str(num_str) {
        Ok(bi) => bi,
        Err(e) => return Err(MyError::ParseBigDecimalError(e)),
    };
    match bigdecimal.to_f64() {
        Some(res) => Ok(res),
        None => Err(MyError::ParseF64Error(num_str.to_string())),
    }
}

fn scientific2dec_u64(num_str: &str) -> Result<u64, MyError> {
    let bigdecimal = match BigDecimal::from_str(num_str) {
        Ok(bi) => bi,
        Err(e) => return Err(MyError::ParseBigDecimalError(e)),
    };
    match bigdecimal.to_u64() {
        Some(res) => Ok(res),
        None => Err(MyError::ParseU64Error(num_str.to_string())),
    }
}

pub fn bigdecimal_from_str(format: NumFormat, num_str: &str) -> Result<BigDecimal, MyError> {
    let num_str = &num_str.replace(",", "").replace("_", "");
    match format {
        NumFormat::Scientific => match BigDecimal::from_str(num_str) {
            Ok(num) => Ok(num),
            Err(e) => Err(MyError::ParseBigDecimalError(e)),
        },
        NumFormat::Hex => {
            let num = hex2dec_u64(&num_str[2..])?;
            Ok(BigDecimal::from(num))
        }
        NumFormat::Oct => {
            let num = oct2dec_u64(&num_str[1..])?;
            Ok(BigDecimal::from(num))
        }
        NumFormat::Bin => {
            let num = bin2dec_u64(&num_str[2..])?;
            Ok(BigDecimal::from(num))
        }
        NumFormat::Dec | NumFormat::DecInt => match BigDecimal::from_str(num_str) {
            Ok(num) => Ok(num),
            Err(e) => Err(MyError::ParseBigDecimalError(e)),
        },
    }
}

pub fn f64_from_str(format: NumFormat, num_str: &str) -> Result<f64, MyError> {
    let num_str = &num_str.replace(",", "").replace("_", "");
    match format {
        NumFormat::Scientific => scientific2dec_f64(num_str),
        NumFormat::Hex => hex2dec_f64(&num_str[2..]),
        NumFormat::Oct => oct2dec_f64(&num_str[1..]),
        NumFormat::Bin => bin2dec_f64(&num_str[2..]),
        NumFormat::Dec | NumFormat::DecInt => match f64::from_str(num_str) {
            Ok(num) => Ok(num),
            Err(e) => Err(MyError::ParseFloatError(e)),
        },
    }
}

pub fn u64_from_str(format: NumFormat, num_str: &str) -> Result<u64, MyError> {
    let num_str = &num_str.replace(",", "").replace("_", "");
    match format {
        NumFormat::Scientific => scientific2dec_u64(num_str),
        NumFormat::Hex => hex2dec_u64(&num_str[2..]),
        NumFormat::Oct => oct2dec_u64(&num_str[1..]),
        NumFormat::Bin => bin2dec_u64(&num_str[2..]),
        NumFormat::Dec => Err(MyError::UnexpectedInput(
            "u64".to_string(),
            num_str.to_string(),
        )),
        NumFormat::DecInt => match u64::from_str(num_str) {
            Ok(num) => Ok(num),
            Err(e) => Err(MyError::ParseIntError(e)),
        },
    }
}

#[cfg(test)]
mod test {

    use bigdecimal::BigDecimal;
    use std::io::Write;
    use text_colorizer::*;

    struct TestCaseBigDecimal {
        num_str: String,
        num_format: super::NumFormat,
        result: BigDecimal,
    }

    struct TestCaseU64 {
        num_str: String,
        num_format: super::NumFormat,
        result: u64,
    }

    struct TestCaseF64 {
        num_str: String,
        num_format: super::NumFormat,
        result: f64,
    }

    macro_rules! test_something_from_str {
        ($tc:ident, $method:ident) => {
            let mut test_success = 0;
            for (i, tc) in $tc.iter().enumerate() {
                match super::$method(tc.num_format, &tc.num_str) {
                    Ok(res) => {
                        if res == tc.result {
                            writeln!(
                                &mut std::io::stderr(),
                                "testcase {}: {} {}",
                                i,
                                "SUCCESSED          ".green(),
                                tc.num_str
                            )
                            .unwrap();
                            test_success += 1;
                        } else {
                            writeln!(
                                &mut std::io::stderr(),
                                "testcase {}: {} {} -> {}, but expected {}",
                                i,
                                "NOT MATCH          ".red(),
                                tc.num_str,
                                res,
                                tc.result
                            )
                            .unwrap();
                        }
                    }
                    Err(e) => {
                        writeln!(
                            &mut std::io::stderr(),
                            "testcase {}: {} {}, '{}'",
                            i,
                            "CONVERSION FAILED  ".red(),
                            tc.num_str,
                            e
                        )
                        .unwrap();
                    }
                }
            }
            writeln!(
                &mut std::io::stderr(),
                "testcase {}/{} SUCCESSED",
                test_success,
                $tc.len(),
            )
            .unwrap();
            assert_eq!(test_success, $tc.len());
        };
    }

    #[test]
    fn test_bigdecimal_from_str() {
        let mut test_cases = Vec::new();
        test_cases.push(TestCaseBigDecimal {
            num_str: "1.16E-6".to_string(),
            num_format: crate::tokenizer::NumFormat::Scientific,
            result: BigDecimal::from(116) / BigDecimal::from(100000000),
        });
        test_cases.push(TestCaseBigDecimal {
            num_str: "0x1234".to_string(),
            num_format: crate::tokenizer::NumFormat::Hex,
            result: BigDecimal::from(4660),
        });
        test_cases.push(TestCaseBigDecimal {
            num_str: "01234".to_string(),
            num_format: crate::tokenizer::NumFormat::Oct,
            result: BigDecimal::from(668),
        });
        test_cases.push(TestCaseBigDecimal {
            num_str: "0b1010".to_string(),
            num_format: crate::tokenizer::NumFormat::Bin,
            result: BigDecimal::from(10),
        });
        test_cases.push(TestCaseBigDecimal {
            num_str: "0.1234".to_string(),
            num_format: crate::tokenizer::NumFormat::Dec,
            result: BigDecimal::from(1234) / BigDecimal::from(10000),
        });
        test_cases.push(TestCaseBigDecimal {
            num_str: "1".to_string(),
            num_format: crate::tokenizer::NumFormat::DecInt,
            result: BigDecimal::from(1),
        });
        test_something_from_str!(test_cases, bigdecimal_from_str);
    }

    #[test]
    fn test_f64_from_str() {
        let mut test_cases = Vec::new();
        test_cases.push(TestCaseF64 {
            num_str: "1.16E-6".to_string(),
            num_format: crate::tokenizer::NumFormat::Scientific,
            result: 0.00000116,
        });
        test_cases.push(TestCaseF64 {
            num_str: "0x1234".to_string(),
            num_format: crate::tokenizer::NumFormat::Hex,
            result: 4660.,
        });
        test_cases.push(TestCaseF64 {
            num_str: "01234".to_string(),
            num_format: crate::tokenizer::NumFormat::Oct,
            result: 668.,
        });
        test_cases.push(TestCaseF64 {
            num_str: "0b1010".to_string(),
            num_format: crate::tokenizer::NumFormat::Bin,
            result: 10.,
        });
        test_cases.push(TestCaseF64 {
            num_str: "0.1234".to_string(),
            num_format: crate::tokenizer::NumFormat::Dec,
            result: 0.1234,
        });
        test_cases.push(TestCaseF64 {
            num_str: "1".to_string(),
            num_format: crate::tokenizer::NumFormat::DecInt,
            result: 1.,
        });
        test_something_from_str!(test_cases, f64_from_str);
    }

    #[test]
    fn test_u64_from_str() {
        let mut test_cases = Vec::new();
        test_cases.push(TestCaseU64 {
            num_str: "1.16E+6".to_string(),
            num_format: crate::tokenizer::NumFormat::Scientific,
            result: 1160000,
        });
        test_cases.push(TestCaseU64 {
            num_str: "0x1234".to_string(),
            num_format: crate::tokenizer::NumFormat::Hex,
            result: 4660,
        });
        test_cases.push(TestCaseU64 {
            num_str: "01234".to_string(),
            num_format: crate::tokenizer::NumFormat::Oct,
            result: 668,
        });
        test_cases.push(TestCaseU64 {
            num_str: "0b1010".to_string(),
            num_format: crate::tokenizer::NumFormat::Bin,
            result: 10,
        });
        test_cases.push(TestCaseU64 {
            num_str: "1".to_string(),
            num_format: crate::tokenizer::NumFormat::DecInt,
            result: 1,
        });
        test_something_from_str!(test_cases, u64_from_str);
    }
}
