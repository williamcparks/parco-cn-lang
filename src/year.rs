use std::{fmt::Display, time::SystemTime};

use time::OffsetDateTime;

/// A wrapper i32 that can represent a year and be formatted with %yy or %yyyy
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Year {
    pub value: i32,
}

impl Year {
    /// construct a year from [`SystemTime`]
    pub fn now() -> Self {
        let now = SystemTime::now();
        let datetime = OffsetDateTime::from(now);
        Self {
            value: datetime.year(),
        }
    }

    /// format with %yy
    pub fn yy(self) -> TwoYear {
        TwoYear { value: self.value }
    }

    /// format with %yyyy
    pub fn yyyy(self) -> FourYear {
        FourYear { value: self.value }
    }
}

pub struct TwoYear {
    value: i32,
}

impl Display for TwoYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value % 100)
    }
}

pub struct FourYear {
    value: i32,
}

impl Display for FourYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
