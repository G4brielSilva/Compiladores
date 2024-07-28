use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Struct,
    Instance,
    ClassType,
    Inheritance,
    Visibility,
    Scope,
    Final,
    Type,
    Value,
    Array,
    Separator,
    Command,
    Else,
    Atrib,
    Switch,
    New,
    Operator,
    ParamList,
    LogicOperator,
    MathOperator,
    This,
    ArrayBracket,
    Parenthesis,
    Field,
    String,
    Char,
    Bloc,
    Identifier,
    Case,
    Number,
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Struct => write!(f, "Struct"),
            Token::Instance => write!(f, "Instance"),
            Token::ClassType => write!(f, "ClassType"),
            Token::Inheritance => write!(f, "Inheritance"),
            Token::Visibility => write!(f, "Visibility"),
            Token::Scope => write!(f, "Scope"),
            Token::Final => write!(f, "Final"),
            Token::Type => write!(f, "Type"),
            Token::Value => write!(f, "Value"),
            Token::Array => write!(f, "Array"),
            Token::Separator => write!(f, "Separator"),
            Token::Command => write!(f, "Command"),
            Token::Else => write!(f, "Else"),
            Token::Atrib => write!(f, "Atrib"),
            Token::Switch => write!(f, "Switch"),
            Token::New => write!(f, "New"),
            Token::Operator => write!(f, "Operator"),
            Token::ParamList => write!(f, "ParamList"),
            Token::LogicOperator => write!(f, "LogicOperator"),
            Token::MathOperator => write!(f, "MathOperator"),
            Token::This => write!(f, "This"),
            Token::ArrayBracket => write!(f, "ArrayBracket"),
            Token::Parenthesis => write!(f, "Parenthesis"),
            Token::Field => write!(f, "Field"),
            Token::String => write!(f, "String"),
            Token::Char => write!(f, "Char"),
            Token::Bloc => write!(f, "Bloc"),
            Token::Identifier => write!(f, "Identifier"),
            Token::Case => write!(f, "Case"),
            Token::Number => write!(f, "Number"),
            Token::Error => write!(f, "Error"),
        }
    }
}

impl PartialEq<&str> for Token {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}