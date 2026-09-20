use bon::Builder;
use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Serialize, Deserialize, Debug, PartialEq, Builder, Clone)]
#[builder(on(_, into))]
#[serde(rename = "SESSION")]
pub struct Session {
    #[builder(start_fn)]
    #[serde(rename = "@number")]
    pub number: u32,

    #[builder(start_fn)]
    #[serde(with = "crate::format::date", rename = "@date")]
    pub date: Date,
}

impl<S: session_builder::IsComplete> From<SessionBuilder<S>> for Session {
    fn from(b: SessionBuilder<S>) -> Session {
        b.build()
    }
}

pub mod sessions_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    type InnerType = super::Session;

    #[derive(Serialize)]
    #[serde(rename = "SESSIONS")]
    struct Borrowed<'a> {
        #[serde(rename = "SESSION")]
        inner: &'a [InnerType],
    }

    #[derive(Deserialize)]
    #[serde(rename = "SESSIONS")]
    struct Owned {
        #[serde(rename = "SESSION")]
        inner: Vec<InnerType>,
    }

    pub fn serialize<S: Serializer>(inner: &[InnerType], serializer: S) -> Result<S::Ok, S::Error> {
        Borrowed { inner }.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<InnerType>, D::Error> {
        Ok(Owned::deserialize(deserializer)?.inner)
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;
    use time::{Date, Month};

    use super::Session;

    const MINIMAL: &str = r#"<SESSION number="1" date="2025-01-02"/>"#;

    fn from_str(str: &str) -> Result<Session, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Session>(str)
    }

    #[test]
    fn builder() {
        let _ = Session::builder(
            1_u32,
            Date::from_calendar_date(2025, Month::January, 2).unwrap(),
        )
        .build();
    }

    #[test]
    fn required_attributes() {
        from_str(r#"<SESSION date="2025-01-02"/>"#).expect_err("SESSION should have a number");
        from_str(r#"<SESSION number="1"/>"#).expect_err("SESSION should have a date");
    }

    #[test]
    fn optional_attributes() {
        let parsed = from_str(MINIMAL).expect("SESSION is valid");
        assert_eq!(
            parsed,
            Session {
                number: 1,
                date: Date::from_calendar_date(2025, Month::January, 2).unwrap(),
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());
    }
}
