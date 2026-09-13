use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Builder, Clone)]
#[serde(rename = "CONTACT")]
pub struct Contact {
    #[builder(start_fn, into)]
    #[serde(rename = "@email")]
    pub email: String,

    #[builder(into)]
    #[serde(rename = "@city", default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[builder(into)]
    #[serde(rename = "@country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[builder(into)]
    #[serde(rename = "@fax", default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,

    #[builder(into)]
    #[serde(rename = "@internet", default, skip_serializing_if = "Option::is_none")]
    pub internet: Option<String>,

    #[builder(into)]
    #[serde(rename = "@name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[builder(into)]
    #[serde(rename = "@mobile", default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,

    #[builder(into)]
    #[serde(rename = "@phone", default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[builder(into)]
    #[serde(rename = "@state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[builder(into)]
    #[serde(rename = "@street", default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    #[builder(into)]
    #[serde(rename = "@street2", default, skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,

    #[builder(into)]
    #[serde(rename = "@zip", default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl<S: contact_builder::IsComplete> From<ContactBuilder<S>> for Contact {
    fn from(b: ContactBuilder<S>) -> Contact {
        b.build()
    }
}

#[cfg(test)]
pub mod tests {
    use quick_xml::se::to_string;

    use super::Contact;

    pub const MINIMAL: &str = r#"<CONTACT email="a@b.c"/>"#;

    fn from_str(str: &str) -> Result<Contact, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Contact>(str)
    }

    #[test]
    fn builder() {
        let _ = Contact::builder("a@b.c")
            .city("city")
            .country("country")
            .fax("fax")
            .internet("internet")
            .name("name")
            .mobile("mobile")
            .phone("phone")
            .state("state")
            .street("street")
            .street2("street2")
            .zip("zip")
            .build();
    }

    #[test]
    fn required_attributes() {
        from_str(r#"<CONTACT city="Montréal"/>"#).expect_err("CONTACT should have an email");
    }

    #[test]
    fn optional_attributes() {
        let parsed = from_str(MINIMAL).expect("CONTACT is valid");
        assert_eq!(
            parsed,
            Contact {
                email: "a@b.c".to_string(),
                ..Contact::default()
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());

        const WITH_OPTIONALS: &str = r#"<CONTACT email="a@b.c" city="Montréal" country="CAN" fax="514-555-0101" internet="https://example.org" name="Jane Doe" mobile="514-555-0102" phone="514-555-0103" state="QC" street="1 Main St" street2="Apt 2" zip="H0H0H0"/>"#;
        let parsed = from_str(WITH_OPTIONALS).expect("CONTACT is valid");
        assert_eq!(Some("Montréal".to_string()), parsed.city);
        assert_eq!(Some("CAN".to_string()), parsed.country);
        assert_eq!(Some("514-555-0101".to_string()), parsed.fax);
        assert_eq!(Some("https://example.org".to_string()), parsed.internet);
        assert_eq!(Some("Jane Doe".to_string()), parsed.name);
        assert_eq!(Some("514-555-0102".to_string()), parsed.mobile);
        assert_eq!(Some("514-555-0103".to_string()), parsed.phone);
        assert_eq!(Some("QC".to_string()), parsed.state);
        assert_eq!(Some("1 Main St".to_string()), parsed.street);
        assert_eq!(Some("Apt 2".to_string()), parsed.street2);
        assert_eq!(Some("H0H0H0".to_string()), parsed.zip);
        assert_eq!(WITH_OPTIONALS, to_string(&parsed).unwrap());
    }
}
