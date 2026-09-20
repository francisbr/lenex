use bon::Builder;
use serde::{Deserialize, Serialize};

pub mod contact;

use contact::Contact;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Builder, Clone)]
#[builder(on(_, into))]
#[serde(rename = "CONSTRUCTOR")]
pub struct Constructor {
    #[builder(start_fn)]
    #[serde(rename = "@name")]
    pub name: String,

    #[builder(start_fn)]
    #[serde(rename = "@version")]
    pub version: String,

    #[builder(start_fn)]
    #[serde(rename = "CONTACT")]
    pub contact: Contact,

    #[serde(
        rename = "@registration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub registration: Option<String>,
}

impl<S: constructor_builder::IsComplete> From<ConstructorBuilder<S>> for Constructor {
    fn from(b: ConstructorBuilder<S>) -> Constructor {
        b.build()
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;

    use super::{Constructor, Contact};

    pub const MINIMAL: &str =
        r#"<CONSTRUCTOR name="n" version="1.0"><CONTACT email="a@b.c"/></CONSTRUCTOR>"#;

    fn from_str(str: &str) -> Result<Constructor, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Constructor>(str)
    }

    #[test]
    fn builder() {
        let _ = Constructor::builder("lenex-rs", "0.0.1", Contact::builder("a@b.c"))
            .registration("registration")
            .build();
    }

    #[test]
    fn required_attributes() {
        from_str(r#"<CONSTRUCTOR version="1.0"><CONTACT email="a@b.c"/></CONSTRUCTOR>"#)
            .expect_err("CONSTRUCTOR should have a name");

        from_str(r#"<CONSTRUCTOR name="n"><CONTACT email="a@b.c"/></CONSTRUCTOR>"#)
            .expect_err("CONSTRUCTOR should have a version");

        from_str(r#"<CONSTRUCTOR name="n" version="1.0"></CONSTRUCTOR>"#)
            .expect_err("CONSTRUCTOR should have a CONTACT");
    }

    #[test]
    fn optional_attributes() {
        let parsed = from_str(MINIMAL).expect("CONSTRUCTOR is valid");
        assert_eq!(
            parsed,
            Constructor {
                name: "n".to_string(),
                registration: None,
                version: "1.0".to_string(),
                contact: Contact {
                    email: "a@b.c".to_string(),
                    ..Contact::default()
                },
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());

        const WITH_REGISTRATION: &str = r#"<CONSTRUCTOR name="n" registration="n" version="1.0"><CONTACT email="a@b.c"/></CONSTRUCTOR>"#;
        let parsed = from_str(WITH_REGISTRATION).expect("CONSTRUCTOR is valid");
        assert_eq!(Some("n".to_string()), parsed.registration);
    }
}
