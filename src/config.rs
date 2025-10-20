use crate::error::*;
use bigdecimal::{BigDecimal, FromPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use text_colorizer::*;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstFormat {
    Tree,
    Sexpr,
    Both,
    None,
}

impl fmt::Display for AstFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstFormat::Tree => write!(f, "Tree"),
            AstFormat::Sexpr => write!(f, "Tree"),
            AstFormat::Both => write!(f, "Both"),
            AstFormat::None => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TrigFuncArg {
    Radian,
    Degree,
}

impl fmt::Display for TrigFuncArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrigFuncArg::Radian => write!(f, "radian"),
            TrigFuncArg::Degree => write!(f, "degree"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub debug: bool,                // デバッグ出力の有無
    pub ast_format: AstFormat,      // ASTのフォーマット
    pub trig_func_arg: TrigFuncArg, // 三角関数の引数, 逆三角関数の結果
    pub log_base: BigDecimal,       // logの底
    pub num_of_digit: u32,          // 結果の小数点以下の桁数
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:
    {: <14}: {}
    {: <14}: {}
    {: <14}: {}
    {: <14}: {}
    {: <14}: {}",
            "Config".green(),
            "debug".cyan(),
            self.debug,
            "ast_format".cyan(),
            self.ast_format,
            "trig_func_arg".cyan(),
            self.trig_func_arg,
            "log_base".cyan(),
            self.log_base,
            "num_of_digit".cyan(),
            self.num_of_digit
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            ast_format: AstFormat::Both,
            trig_func_arg: TrigFuncArg::Radian,
            log_base: BigDecimal::from_f64(std::f64::consts::E).unwrap(),
            num_of_digit: 12,
        }
    }
}

impl Config {
    pub fn load_from_file(&mut self) -> Result<PathBuf, MyError> {
        let mut conf_file = Self::config_dir(false)?;
        conf_file.push("config.toml");
        if conf_file.exists() {
            let conf_str = fs::read_to_string(&conf_file).unwrap();
            let config: Result<Config, toml::de::Error> = toml::from_str(&conf_str);
            match config {
                Ok(c) => {
                    *self = c;
                    Ok(conf_file)
                }
                Err(e) => Err(MyError::TomlDeserializeError(e)),
            }
        } else {
            Err(MyError::NoConfigErr(format!("{conf_file:?}")))
        }
    }

    pub fn write_to_file(&self) -> Result<PathBuf, MyError> {
        let mut conf_file = Self::config_dir(true)?;
        conf_file.push("config.toml");
        let mut config_file = File::create(&conf_file).unwrap();
        let config_toml = toml::to_string(self).expect("couldn't serialize config");
        write!(config_file, "{config_toml}").expect("couldn't write config to config.toml");
        config_file.flush().expect("failed flush file I/O");
        Ok(conf_file)
    }

    fn config_dir(is_create: bool) -> Result<PathBuf, MyError> {
        if let Some(mut conf_path) = dirs::home_dir() {
            #[cfg(target_family = "unix")]
            {
                conf_path.push(".config");
            }
            #[cfg(target_family = "windows")]
            {
                conf_path.push("AppData");
                conf_path.push("Roaming");
            }
            conf_path.push("tsc");

            if conf_path.exists() {
                Ok(conf_path)
            } else if is_create {
                match fs::create_dir(&conf_path) {
                    Ok(()) => Ok(conf_path),
                    Err(e) => Err(MyError::ConfigWriteErr(format!(
                        "couldn't create {conf_path:?}, {e}"
                    ))),
                }
            } else {
                Err(MyError::ConfigLoadErr(format!("not found {conf_path:?}")))
            }
        } else {
            Err(MyError::ConfigLoadErr("couldn't get home dir".to_string()))
        }
    }
}

pub fn config_reader() -> Result<std::sync::RwLockReadGuard<'static, Config>, MyError> {
    match crate::CONFIG.try_read() {
        Ok(c) => Ok(c),
        Err(e) => Err(MyError::ConfigReadErr(e.to_string())),
    }
}

pub fn config_writer() -> Result<std::sync::RwLockWriteGuard<'static, Config>, MyError> {
    match crate::CONFIG.try_write() {
        Ok(c) => Ok(c),
        Err(e) => Err(MyError::ConfigWriteErr(e.to_string())),
    }
}
