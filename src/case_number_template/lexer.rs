use crate::CaseNumberTemplatePart;

pub struct Lexer<'a> {
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    pub fn part(&mut self) -> Option<CaseNumberTemplatePart<'a>> {
        if self.src.is_empty() {
            return None;
        }

        if let Some(stripped) = self.src.strip_prefix("%yy") {
            let (rest, res) = match stripped.strip_prefix("yy") {
                Some(rest) => (rest, CaseNumberTemplatePart::FourYear),
                None => (stripped, CaseNumberTemplatePart::TwoYear),
            };

            self.src = rest;

            Some(res)
        } else if let Some(stripped) = self.src.strip_prefix('*') {
            let mut chars = stripped.chars();
            let (digit, len) = match chars
                .next()
                .and_then(|ch| ch.to_digit(10).map(|v| (v, ch.len_utf8())))
            {
                Some(some) => some,
                None => {
                    self.src = stripped;

                    return Some(CaseNumberTemplatePart::Constant("*"));
                }
            };
            if chars.next() != Some('n') {
                self.src = stripped;

                return Some(CaseNumberTemplatePart::Constant("*"));
            }

            let idx = '*'.len_utf8() + len + 'n'.len_utf8();
            self.src = match self.src.get(idx..) {
                Some(some) => some,
                None => {
                    self.src = stripped;

                    return Some(CaseNumberTemplatePart::Constant("*"));
                }
            };

            Some(CaseNumberTemplatePart::Numeric(digit as usize))
        } else {
            for (idx, ch) in self.src.char_indices() {
                if ch == '%' || ch == '*' {
                    let (substr, rest) = self.src.split_at(idx);
                    self.src = rest;
                    return Some(CaseNumberTemplatePart::Constant(substr));
                }
            }

            let res = CaseNumberTemplatePart::Constant(self.src);

            self.src = "";

            Some(res)
        }
    }
}
