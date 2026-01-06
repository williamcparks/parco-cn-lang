use crate::CaseNumberTemplate;

/// the parts of a template
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaseNumberTemplatePart<'a> {
    /// Some constant such as "JP"
    Constant(&'a str),
    /// Some constant that is owned such as "JP"
    ConstantOwned(Box<str>),
    /// A two year specifier: "%yy"
    TwoYear,
    /// a four year specifier: "%yyyy"
    FourYear,
    /// Some numeric template such as "*8n" where `8` would be the `usize` in this case
    Numeric(usize),
}

impl<'a> CaseNumberTemplatePart<'a> {
    pub fn into_owned(self) -> CaseNumberTemplatePart<'static> {
        match self {
            Self::Constant(s) => CaseNumberTemplatePart::ConstantOwned(s.into()),
            Self::ConstantOwned(boxed) => CaseNumberTemplatePart::ConstantOwned(boxed),
            Self::TwoYear => CaseNumberTemplatePart::TwoYear,
            Self::FourYear => CaseNumberTemplatePart::FourYear,
            Self::Numeric(num) => CaseNumberTemplatePart::Numeric(num),
        }
    }
}

impl<'a> CaseNumberTemplate<'a> {
    /// get the individual parts that make up the template
    pub fn into_parts(self) -> Vec<CaseNumberTemplatePart<'a>> {
        self.parts
    }

    /// get the individual parts that make up the template as a slice
    pub fn parts(&self) -> &[CaseNumberTemplatePart<'a>] {
        self.parts.as_slice()
    }
}
