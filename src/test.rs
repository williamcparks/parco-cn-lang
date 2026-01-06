use crate::{CaseNumberTemplate, CaseNumberTemplatePart, Year};

#[test]
fn test() {
    let template = CaseNumberTemplate::new("JP01-%yy-E*8n");

    assert_eq!(
        serde_json::to_string(&template).unwrap(),
        "\"JP01-%yy-E*8n\""
    );

    let other_template: CaseNumberTemplate = serde_json::from_str("\"JP01-%yy-E*8n\"").unwrap();

    assert_eq!(template, other_template);

    let year = Year::now();
    let case_number = template.case_number(123, year);
    let expected_case_number = format!("JP01-{}-E00000123", year.yy());

    assert_eq!(case_number.value.as_ref(), expected_case_number.as_str());

    dbg!(case_number);

    assert_eq!(
        template.into_parts(),
        vec![
            CaseNumberTemplatePart::Constant("JP01-"),
            CaseNumberTemplatePart::TwoYear,
            CaseNumberTemplatePart::Constant("-E"),
            CaseNumberTemplatePart::Numeric(8)
        ]
    );
}
