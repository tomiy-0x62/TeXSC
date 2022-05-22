use regex::Regex;

pub enum TokenType {
    TexCommand,
    Operator,
    Var,
    Num,
    Brace,
}

pub struct Token {
    pub token: String,
    pub token_type: TokenType,
}

pub struct Lexer {
    formulas: String,
    pub tokens: Vec<Token>
}

/*
tokenの種類
- 数値リテラル: 0x54, 0b100011, 534
- TeXコマンド: \log, \sum
- コマンド: sin, cos
- かっこ: (, ), {, }
- 変数: x, y
*/

impl Lexer {
    pub fn new(form: String) -> Lexer {
        let form = form.replace("\n", "").replace("\t", "");
        Lexer { formulas: form, tokens: Vec::new() }
    }

    pub fn print_form(&self) {
        println!("form: {}", self.formulas.replace("\n", " "));
    }

    fn print_token(&self) {
        for token in self.tokens.iter() {
            // {}でした
            print!("'{}', ", token.token);
        }
        println!("");
    }

    pub fn analyze(&mut self) {
        let tex_command = Regex::new(r"\\[A-Za-z]*").unwrap(); // OK
        let operator = Regex::new(r"\+|-|\*|=|/|!|_|,|\^|\|").unwrap(); // OK
        let var = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap(); // OK
        let num = Regex::new(r"0x[0-9a-fA-F]+|0b[0-1]+|[0-9]+\.?[0-9]*").unwrap(); // OK
        let braces = Regex::new(r"\(|\)|\[|\]|\{|\}").unwrap(); // OK
        // let token_types: Vec<Regex> = [tex_command, operator, command, var, num, braces].to_vec();

        loop {
            // TODO: 0b423 -> num:"0", var"b423"と分割失敗してるのを修正
            // 0b423みたいなのがきたらエラーにしたい
            // TODO: a\sindsをどう扱うか決める -> 'a', '\sin', 'ds' or '\sinds'(構文解析のときにpanic)
            let mut c = self.formulas.chars().nth(0).unwrap();
            if c == ' ' {
                self.formulas = self.formulas.replacen(" ", "", 1);
                c = self.formulas.chars().nth(0).unwrap();
            }
            let mut ismatch = false;
            if c == '\\' {
                if let Some(caps) = tex_command.captures(&self.formulas) {
                    // println!("<<< match '{}' as tex_command >>>", caps.get(0).unwrap().as_str());
                    self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::TexCommand});
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                    ismatch = true;
                }
            } else if let Some(caps) = operator.captures(&c.to_string()) {
                // println!("<<< match '{}' as operator >>>", caps.get(0).unwrap().as_str());
                self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Operator});
                self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                ismatch = true;
            } else if let Some(caps) = braces.captures(&c.to_string()) {
                // println!("<<< match '{}' as braces >>>", caps.get(0).unwrap().as_str());
                self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Brace});
                self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                ismatch = true;
            } else if let Some(_) = num.captures(&c.to_string()) {
                if let Some(caps) = num.captures(&self.formulas) {
                    // println!("<<< match '{}' as num >>>", caps.get(0).unwrap().as_str());
                    self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Num});
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                    ismatch = true;
                }
            } else if let Some(caps) = var.captures(&self.formulas) {
                if c != caps.get(0).unwrap().as_str().chars().nth(0).unwrap() { continue; }
                    // println!("<<< match '{}' as var >>>", caps.get(0).unwrap().as_str());
                    self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Var});
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                    ismatch = true;
            } 
            if !ismatch {
                panic!("hoge")
            }

            // println!("formulas: '{}'", self.formulas);

            if self.formulas.len() == 0 {
                self.print_token();
                break;
            }
        }

        
        /*
        for caps in tex_command.captures_iter(&self.formulas) {
            println!("match '{}'", &caps[0]);
        }*/
        
        if let Some(caps) = operator.captures(&self.formulas) {
            println!("<<< match '{}' >>>", caps.get(0).unwrap().as_str());
            // if let Some(hoge) = caps.get(0)
        }

    }
}
