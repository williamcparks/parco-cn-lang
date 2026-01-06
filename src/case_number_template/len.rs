use crate::{CaseNumberTemplate, CaseNumberTemplatePart};

impl<'a> CaseNumberTemplate<'a> {
    /// get the sum of the lengths of the [`CaseNumberTemplatePart`]
    pub fn len(&self) -> usize {
        self.parts.iter().map(CaseNumberTemplatePart::len).sum()
    }

    /// check if all [`CaseNumberTemplatePart`] are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> CaseNumberTemplatePart<'a> {
    /// get the length of the individual part if it were rendered
    pub fn len(&self) -> usize {
        match self {
            Self::Constant(s) => s.len(),
            Self::ConstantOwned(s) => s.len(),
            Self::TwoYear => 2,
            Self::FourYear => 4,
            Self::Numeric(n) => *n,
        }
    }

    /// check if the part is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
