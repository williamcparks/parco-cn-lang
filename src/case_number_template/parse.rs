use std::{convert::Infallible, str::FromStr};

use crate::{CaseNumberTemplate, case_number_template::lexer::Lexer};

impl<'a> CaseNumberTemplate<'a> {
    /// parse a new case number template borrowing from the input
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);

        let mut parts = Vec::new();
        while let Some(part) = lexer.part() {
            parts.push(part);
        }

        Self { parts }
    }

    /// parse a new case number template *WITHOUT* borrowing from the input
    pub fn new_owned(input: &str) -> Self {
        let mut lexer = Lexer::new(input);

        let mut parts = Vec::new();
        while let Some(part) = lexer.part() {
            parts.push(part.into_owned());
        }

        Self { parts }
    }
}

impl<'a> FromStr for CaseNumberTemplate<'a> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new_owned(s))
    }
}
