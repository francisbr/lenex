use bon::Builder;
use serde::{Deserialize, Serialize};

pub mod constructor;
pub mod meet;

use constructor::Constructor;
use meet::Meet;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Builder, Clone)]
#[serde(rename = "LENEX")]
pub struct Lenex {
    #[builder(start_fn, into)]
    #[serde(rename = "@version")]
    pub version: String,

    #[builder(start_fn, into)]
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Constructor,

    #[builder(into, default)]
    #[serde(
        rename = "MEETS",
        with = "meet::meets_serde",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    meets: Vec<Meet>,
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;

    use crate::model::{
        Lenex,
        constructor::{Constructor, contact::Contact},
    };

    const MINIMAL: &str = r#"<LENEX version="3.0"><CONSTRUCTOR name="n" version="1.0"><CONTACT email="a@b.c"/></CONSTRUCTOR></LENEX>"#;

    fn from_str(str: &str) -> Result<Lenex, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Lenex>(str)
    }

    #[test]
    fn builder() {
        let _ = Lenex::builder(
            "3.0",
            Constructor::builder("lenex-rs", "0.0.1", Contact::builder("a@b.c")),
        )
        .build();
    }

    #[test]
    fn required_attributes() {
        from_str(
            r#"<LENEX><CONSTRUCTOR name="n" version="1.0"><CONTACT email="a@b.c"/></CONSTRUCTOR></LENEX>"#,
        )
        .expect_err("LENEX should have a version");

        from_str(r#"<LENEX version="3.0"></LENEX>"#).expect_err("LENEX should have a CONSTRUCTOR");
    }

    #[test]
    fn optional_attributes() {
        let parsed = from_str(MINIMAL).expect("LENEX is valid");
        assert_eq!(
            parsed,
            Lenex {
                version: "3.0".to_string(),
                constructor: Constructor {
                    name: "n".to_string(),
                    registration: None,
                    version: "1.0".to_string(),
                    contact: Contact {
                        email: "a@b.c".to_string(),
                        ..Contact::default()
                    },
                },
                ..Default::default()
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());

        const WITH_MEETS: &str = r#"<LENEX version="3.0"><CONSTRUCTOR name="n" version="1.0"><CONTACT email="a@b.c"/></CONSTRUCTOR><MEETS><MEET name="m1" city="Montreal" nation="CAN"><SESSIONS><SESSION number="1" date="2025-01-02"/></SESSIONS></MEET></MEETS></LENEX>"#;
        let parsed = from_str(WITH_MEETS).expect("LENEX is valid");
        assert_eq!(1, parsed.meets.len());
        assert_eq!(WITH_MEETS, to_string(&parsed).unwrap());
    }
}
