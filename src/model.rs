use bon::Builder;
use serde::{Deserialize, Serialize};

mod constructor;

pub use constructor::{Constructor, ConstructorBuilder, Contact, ContactBuilder};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Builder)]
#[serde(rename = "LENEX")]
pub struct Lenex {
    #[builder(start_fn, into)]
    #[serde(rename = "@version")]
    pub version: String,

    #[builder(start_fn, into)]
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Constructor,
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;

    use crate::{Constructor, Contact};

    use super::Lenex;

    const MINIMAL: &str = r#"<LENEX version="3.0"><CONSTRUCTOR name="n" version="1.0"><CONTACT email="a@b.c"/></CONSTRUCTOR></LENEX>"#;

    fn from_str(str: &str) -> Result<Lenex, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Lenex>(str)
    }

    #[test]
    fn builder() {
        let _ = Lenex::builder(
            "3.0",
            Constructor::builder(Contact::builder("a@b.c"), "lenex-rs", "0.0.1"),
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
        let parsed = from_str(MINIMAL).expect("CONSTRUCTOR is valid");
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
                }
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());

        // @TODO: Implement optional attributes
    }
}
