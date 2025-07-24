use crate::MyError;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};

pub fn pow(base: BigDecimal, exp: BigDecimal) -> Result<BigDecimal, MyError> {
    let base_f64 = base
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), base))?;
    let exp_f64 = exp
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), exp))?;

    let result_f64 = base_f64.powf(exp_f64);

    BigDecimal::from_f64(result_f64).ok_or(MyError::CalcErr(format!("{base_f64}^{{{exp_f64}}}")))
}

pub fn log(base: BigDecimal, antilog: BigDecimal) -> Result<BigDecimal, MyError> {
    let base_f64 = base
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), base))?;
    let antilog_f64 = antilog
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), antilog))?;

    let result_f64 = antilog_f64.log(base_f64);

    BigDecimal::from_f64(result_f64)
        .ok_or(MyError::CalcErr(format!("\\log_{base_f64} {antilog_f64}")))
}

pub fn sin(x: BigDecimal) -> Result<BigDecimal, MyError> {
    let x_f64 = x
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), x))?;
    let result_f64 = x_f64.sin();

    BigDecimal::from_f64(result_f64).ok_or(MyError::CalcErr(format!("\\sin {x_f64}")))
}
pub fn cos(x: BigDecimal) -> Result<BigDecimal, MyError> {
    let x_f64 = x
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), x))?;
    let result_f64 = x_f64.cos();

    BigDecimal::from_f64(result_f64).ok_or(MyError::CalcErr(format!("\\cos {x_f64}")))
}
pub fn tan(x: BigDecimal) -> Result<BigDecimal, MyError> {
    let x_f64 = x
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), x))?;
    let result_f64 = x_f64.tan();

    BigDecimal::from_f64(result_f64).ok_or(MyError::CalcErr(format!("\\tan {x_f64}")))
}
pub fn asin(x: BigDecimal) -> Result<BigDecimal, MyError> {
    let x_f64 = x
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), x))?;
    let result_f64 = x_f64.asin();

    BigDecimal::from_f64(result_f64).ok_or(MyError::CalcErr(format!("\\asin {x_f64}")))
}
pub fn acos(x: BigDecimal) -> Result<BigDecimal, MyError> {
    let x_f64 = x
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), x))?;
    let result_f64 = x_f64.acos();

    BigDecimal::from_f64(result_f64).ok_or(MyError::CalcErr(format!("\\acos {x_f64}")))
}
pub fn atan(x: BigDecimal) -> Result<BigDecimal, MyError> {
    let x_f64 = x
        .to_f64()
        .ok_or(MyError::ConvertErr("f64".to_string(), x))?;
    let result_f64 = x_f64.atan();

    BigDecimal::from_f64(result_f64).ok_or(MyError::CalcErr(format!("\\atan {x_f64}")))
}
