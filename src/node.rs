use crate::enum_token::Token;
use core::fmt;

pub struct Node {
    pub value: String,
    pub token: Token,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Escreva o formato desejado para a exibição do Node
        write!(f, "{} - {}", self.value, self.token)
    }
}