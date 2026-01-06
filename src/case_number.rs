use std::fmt::Write;

use crate::{CaseNumberTemplate, CaseNumberTemplatePart, Year};

/// A case number is rendered [`CaseNumberTemplate`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseNumber {
    /// inner case number value stored as a [`Box`] str
    pub value: Box<str>,
}

impl CaseNumber {
    /// construct a [`CaseNumber`] via [`CaseNumberTemplate`] and a numeric + [`Year`]
    pub fn new(template: &CaseNumberTemplate, numeric: usize, year: Year) -> Self {
        let mut buf = String::with_capacity(template.len());

        for part in template.parts() {
            match part {
                CaseNumberTemplatePart::Constant(s) => buf.push_str(s),
                CaseNumberTemplatePart::ConstantOwned(s) => buf.push_str(s.as_ref()),
                CaseNumberTemplatePart::TwoYear => {
                    let _ = write!(&mut buf, "{}", year.yy());
                }
                CaseNumberTemplatePart::FourYear => {
                    let _ = write!(&mut buf, "{}", year.yyyy());
                }
                CaseNumberTemplatePart::Numeric(num) => {
                    if *num == 0 {
                        continue;
                    }
                    let _ = write!(&mut buf, "{:0width$}", numeric, width = num);
                }
            }
        }

        Self {
            value: buf.into_boxed_str(),
        }
    }
}

impl<'a> CaseNumberTemplate<'a> {
    /// construct a [`CaseNumber`] via [`CaseNumberTemplate`] and a numeric + [`Year`]
    pub fn case_number(&self, numeric: usize, year: Year) -> CaseNumber {
        CaseNumber::new(self, numeric, year)
    }
}
