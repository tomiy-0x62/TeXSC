use crate::error::*;
use crate::CONFIG;
use dirs;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use text_colorizer::*;
use toml;

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

#[derive(Serialize, Deserialize, Clone, Copy)]
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
    pub trig_func_arg: TrigFuncArg, // 三角関数の引数
    pub log_base: f64,              // logの底
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

impl Config {
    pub fn load_from_file(&mut self) -> Result<(), MyError> {
        let mut conf_dir = Self::config_dir(false)?;
        conf_dir.push("config.toml");
        if conf_dir.exists() {
            let conf_str = fs::read_to_string(conf_dir).unwrap();
            let config: Result<Config, toml::de::Error> = toml::from_str(&conf_str);
            match config {
                Ok(c) => {
                    *self = c;
                    Ok(())
                }
                Err(e) => Err(MyError::TomlDeserializeError(e)),
            }
        } else {
            return Err(MyError::NoConfigErr(format!("{:?}", conf_dir)));
        }
    }

    pub fn write_to_file(&self) -> Result<(), MyError> {
        let mut conf_dir = Self::config_dir(true)?;
        conf_dir.push("config.toml");
        let mut config_file = File::create(conf_dir).unwrap();
        let config_toml = toml::to_string(self).expect("couldn't serialize config");
        write!(config_file, "{}", config_toml).expect("couldn't write config to config.toml");
        config_file.flush().expect("failed flush file I/O");
        Ok(())
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
                conf_path.push("Local");
            }
            conf_path.push("tsc");

            if conf_path.exists() {
                Ok(conf_path)
            } else {
                if is_create {
                    match fs::create_dir(conf_path.clone()) {
                        Ok(()) => Ok(conf_path),
                        Err(e) => Err(MyError::ConfigWriteErr(format!(
                            "couldn't create {:?}, {}",
                            conf_path,
                            e.to_string()
                        ))),
                    }
                } else {
                    Err(MyError::ConfigLoadErr(format!("not found {:?}", conf_path)))
                }
            }
        } else {
            Err(MyError::ConfigLoadErr("couldn't get home dir".to_string()))
        }
    }
}

pub fn read_config() -> Result<Config, MyError> {
    let ref conf = match CONFIG.read() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigReadErr(e.to_string())),
    };
    Ok(Config {
        debug: conf.debug,
        ast_format: conf.ast_format,
        trig_func_arg: conf.trig_func_arg,
        log_base: conf.log_base,
        num_of_digit: conf.num_of_digit,
    })
}

pub fn set_dbconf(db: bool) -> Result<(), MyError> {
    let ref mut conf = match CONFIG.write() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigWriteErr(e.to_string())),
    };
    conf.debug = db;
    Ok(())
}

pub fn set_afconf(af: AstFormat) -> Result<(), MyError> {
    let ref mut conf = match CONFIG.write() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigWriteErr(e.to_string())),
    };
    conf.ast_format = af;
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
