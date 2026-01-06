/// a case number template represents the way to construct [`CaseNumbers`](`crate::CaseNumber`)
/// ex: JP01-%yy-E*8n
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseNumberTemplate<'a> {
    parts: Vec<CaseNumberTemplatePart<'a>>,
}

pub use part::CaseNumberTemplatePart;

mod display;
mod len;
mod lexer;
mod parse;
mod part;
mod serde;
