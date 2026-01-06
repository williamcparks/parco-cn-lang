use std::fmt::Display;

use crate::{CaseNumberTemplate, CaseNumberTemplatePart};

impl<'a> Display for CaseNumberTemplate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in self.parts.iter() {
            write!(f, "{part}")?;
        }
        Ok(())
    }
}

impl<'a> Display for CaseNumberTemplatePart<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(s) => f.write_str(s),
            Self::ConstantOwned(boxed) => f.write_str(boxed.as_ref()),
            Self::TwoYear => f.write_str("%yy"),
            Self::FourYear => f.write_str("%yyyy"),
            Self::Numeric(len) => write!(f, "*{len}n"),
        }
    }
}
