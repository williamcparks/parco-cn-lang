#[doc = include_str!("../README.md")]
mod case_number;
mod case_number_template;
mod year;

pub use case_number::CaseNumber;
pub use case_number_template::{CaseNumberTemplate, CaseNumberTemplatePart};
pub use year::Year;

#[cfg(test)]
mod test;
