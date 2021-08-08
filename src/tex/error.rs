//! Error types and error display logic

use crate::tex::token::token;
use crate::tex::token::token::{Token, Value};
use colored::*;
use std::rc::Rc;

#[derive(Debug)]
struct TokenError {
    line: String,
    line_number: isize,
    position: usize,
    width: usize,
    file_description: String,
    message: String,
    notes: Vec<String>,
}

impl std::error::Error for TokenError {}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bar = "|".bright_yellow().bold();
        write!(
            f,
            "{}: {}\n",
            "Error".bright_red().bold(),
            ColoredString::from(self.message.as_str()).bold()
        )?;
        write!(
            f,
            " {} {}:{}:{} \n",
            " >".bright_yellow().bold(),
            "foo.tex",
            self.line_number,
            self.position
        )?;
        write!(f, "  {} \n", bar)?;
        write!(f, "{} {} {} \n", "5".bright_yellow(), bar, self.line)?;
        write!(
            f,
            "  {}                           {}\n",
            bar,
            "^".bright_red().bold()
        )?;
        write!(f, "  {}    \n", bar)?;
        write!(f, "  {} {} expected the escape character to be followed by the name of a control sequence\n",
        "=".bright_yellow().bold(), "note:".bold())
    }
}

pub fn new_token_error(token: Token, message: String, notes: Vec<String>) -> anyhow::Error {
    // TODO: better handling for no source case?
    let source = match token.source {
        None => token::Source {
            line: Rc::new(token::Line {
                content: "".to_string(),
                line_number: 0,
                file: Rc::new("".to_string()),
            }),
            position: 0,
        },
        Some(source) => source,
    };
    anyhow::Error::from(TokenError {
        line: source.line.content.clone(),
        line_number: source.line.line_number,
        position: source.position,
        width: match token.value {
            Value::Character(_, _) => 1,
            Value::ControlSequence(_, name) => 1 + name.len(),
        },
        file_description: "".to_string(), // TODO token.source.line.file.bo).clone(),
        message,
        notes,
    })
}
