use crate::enum_token::Token;
use core::fmt;

#[warn(const_item_mutation)]
pub struct Row {
    pub name: String,
    pub classification: Token,
    pub data_type: String,
    pub scope: String,
    pub qtd: u32,
    pub ord: u32,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Escreva o formato desejado para a exibição do Node
        write!(f, "{} | {} | {} | {} | {} | {}", self.name, self.classification, self.data_type, self.scope, self.qtd, self.ord)
    }
}