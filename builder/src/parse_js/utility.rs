use crate::utility::CommentState::{block, line, none};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

/// `start` should look something like `"var ClassName = function () {"`.
/// The opening `{` is very important!
pub fn get_definition<'a>(start: &'a str, content: &'a str) -> Option<&'a str> {
    let start_index = content.find(start)?;
    let mut brackets = 1;
    let mut end_index = start_index + start.len();
    for c in content.chars().skip(end_index) {
        brackets += match c {
            '{' => 1,
            '}' => -1,
            _ => 0,
        };
        end_index += 1;
        if brackets == 0 {
            break;
        }
    }
    Some(&content[start_index..end_index])
}

pub fn run_rustfmt(source_file: &str) -> Result<(), Box<dyn Error>> {
    let _output = Command::new("rustfmt").arg(source_file).output()?;
    // TODO: check output.status.success()
    Ok(())
}

/// This assumes that the source code is valid, and that every statement ends with a semicolon.
/// Trailing semicolon for function declarations is optional.
pub fn get_next_javascript_statement<'a>(source_code: &'a str) -> (JavascriptStatement, &'a str) {
    let start_index = get_code_index(source_code, 0);
    if start_index >= source_code.len() {
        return (JavascriptStatement::Empty(), "");
    }

    (JavascriptStatement::Empty(), "")
}

enum CommentState {
    none,
    line,
    block,
}

/// Identify index >= start of next character which is nether whitespace nor part of a comment
fn get_code_index(source_code: &str, start: usize, stack: Vec<char>) -> usize {
    // let chars = source_code.chars().skip(start);
    // for z in chars.clone().zip(chars.skip(1)).enumerate() {
    //     match (z, &comment_state, stack.len()) {
    //         // handle comments
    //         ((_, ('/', '*')), none, _) => comment_state = block,
    //         ((_, ('/', '/')), none, _) => comment_state = line,
    //         ((_, ('*', '/')), block, _) => comment_state = none,
    //         ((_, ('\n', _)), line, _) => comment_state = none,
    //         // handle whitespace
    //         ((offset, (c, _)), none, 0) if !c.is_whitespace() => return start+offset,
    //         // we are either in a comment, or in whitespace, or in some m
    //         _=>{}
    //     }
    // }
    let chars = source_code.chars().collect::<Vec<char>>();
    const EXCLUDE: &str = "\"/*";
    for (index, char) in chars.iter().enumerate() {
        match (index, char, stack.last()) {
            // handle start of line comment
            (i, '/', None) if i < chars.len() - 1 && chars[i + 1] == '/' => stack.push('/'),
            (i, '/', Some(&end)) if i < chars.len() - 1 && chars[i + 1] == '/' && !EXCLUDE.contains(end) => stack.push('/'),
            // handle start of block comment
            (i, '/', None) if i < chars.len() - 1 && chars[i + 1] == '*' => stack.push('*'),
            (i, '/', Some(&end)) if i < chars.len() - 1 && chars[i + 1] == '*' && !EXCLUDE.contains(end) => stack.push('/'),
            // handle end of line comment
            (_, '\n', Some('/')) => { stack.pop(); }
            // handle end of block comment
            (i, '*', Some('*')) if i < chars.len() - 1 && chars[i + 1] == '/' => { stack.pop(); }
            // handle start and end of string
            (_, '"', Some('"')) => { stack.pop(); },
            (_, '"', None)
            // handle braces and strings. They can only start if not in comment or string
            // (_, '(', Some(&end)) if !""
        }
    }

    source_code.len()
}

#[inline(never)]
pub fn get_all_javascript_statements(source_code: &str) -> Vec<JavascriptStatement> {
    let mut result = vec![];
    let mut remainder = source_code;

    loop {
        match get_next_javascript_statement(remainder) {
            (JavascriptStatement::Empty(), _) => return result,
            (s, r) => {
                result.push(s);
                remainder = r;
            }
        }
    }
}

pub enum JavascriptStatement {
    Attribute(JAttribute),
    Method(JMethod),
    Class(JClass),
    Variable(JVariable),
    Assigment(JAssigment),
    FunctionCall(JFunctionCall),
    Generic(String),
    Empty(),
}

pub struct JAttribute {
    attribute: String,
    value: JavascriptExpression,
}

pub struct JMethod {
    name: String,
    args: Vec<String>,
    body: String,
}

pub struct JClass {
    name: String,
    constructor: JMethod,
    body: Vec<JavascriptStatement>,
}

pub struct JVariable {
    name: String,
    value: JavascriptExpression,
}

pub enum JavascriptExpression {
    Str(String),
    Int(i64),
    Float(f64),
    Object(HashMap<String, JavascriptExpression>),
    Array(Vec<JavascriptExpression>),
    Generic(String),
}

pub struct JAssigment {
    lhs: String,
    rhs: JavascriptExpression,
}

pub struct JFunctionCall {
    name: String,
    args: Vec<JavascriptExpression>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{read_to_string, BufReader};

    #[test]
    fn test_names() {
        let reader = BufReader::new(File::open("data/card-names.js").unwrap());
        let s = get_all_javascript_statements(read_to_string(reader).unwrap().as_str());
        assert!(s.len() > 0)
    }
}
