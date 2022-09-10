use std::fmt;
use super::CONFIG;
use super::error::*;

#[derive(Clone, Copy)]
pub enum ResultFormat {
    Decimal,
    Binary,
    Hexadecimal,
}

impl fmt::Display for ResultFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultFormat::Decimal =>  write!(f, "decimal"),
            ResultFormat::Binary =>  write!(f, "binary"),
            ResultFormat::Hexadecimal =>  write!(f, "hexadecimal"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TrigFuncArg {
    Radian,
    Degree,
}

impl fmt::Display for TrigFuncArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrigFuncArg::Radian =>  write!(f, "radian"),
            TrigFuncArg::Degree =>  write!(f, "degree"),
        }
    }
}

pub struct Config {
    pub result_format: ResultFormat, // 結果表示のフォーマット
    pub debug: bool, // デバッグ出力の有無
    pub trig_func_arg: TrigFuncArg, // 三角関数の引数
    pub log_base : f64, // logの底
    pub num_of_digit : u32, // 結果の小数点以下の桁数
}

pub fn read_config() -> Result<Config, MyError> {
    let ref conf = match CONFIG.read() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigReadErr(e.to_string())),
    };
    Ok(Config { result_format: conf.result_format, debug: conf.debug, trig_func_arg: conf.trig_func_arg, log_base: conf.log_base, num_of_digit: conf.num_of_digit})
}

pub fn set_rfconf(rf: ResultFormat) -> Result<(), MyError> {
    let ref mut conf = match CONFIG.write() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigWriteErr(e.to_string())),
    };
    conf.result_format = rf;
    Ok(())
}

pub fn set_dbconfig(db: bool) -> Result<(), MyError> {
    let ref mut conf = match CONFIG.write() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigWriteErr(e.to_string())),
    };
    conf.debug = db;
    Ok(())
}

pub fn set_tfconf(tf: TrigFuncArg) -> Result<(), MyError> {
    let ref mut conf = match CONFIG.write() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigWriteErr(e.to_string())),
    };
    conf.trig_func_arg = tf;
    Ok(())
}

pub fn set_lbconf(lb: f64) -> Result<(), MyError> {
    let ref mut conf = match CONFIG.write() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigWriteErr(e.to_string())),
    };
    conf.log_base = lb;
    Ok(())
}

pub fn set_ndconf(nd: u32) -> Result<(), MyError> {
    let ref mut conf = match CONFIG.write() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigWriteErr(e.to_string())),
    };
    conf.num_of_digit = nd;
    Ok(())
}