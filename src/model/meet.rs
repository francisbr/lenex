use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::model::meet::session::Session;

mod session;

#[derive(Serialize, Deserialize, Debug, PartialEq, Builder, Clone)]
#[builder(on(_, into))]
#[serde(rename = "MEET")]
pub struct Meet {
    #[builder(start_fn)]
    #[serde(rename = "@name")]
    name: String,

    #[builder(start_fn)]
    #[serde(rename = "@city")]
    city: String,

    #[builder(start_fn)]
    #[serde(rename = "@nation")]
    nation: String,

    #[builder(start_fn)]
    #[serde(rename = "SESSIONS", with = "session::sessions_serde")]
    sessions: Vec<Session>,
}

pub mod meets_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    type InnerType = super::Meet;

    #[derive(Serialize)]
    #[serde(rename = "MEETS")]
    struct Borrowed<'a> {
        #[serde(rename = "MEET")]
        inner: &'a [InnerType],
    }

    #[derive(Deserialize)]
    #[serde(rename = "MEETS")]
    struct Owned {
        #[serde(rename = "MEET")]
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

    use crate::model::meet::{Meet, session::Session};

    const MINIMAL: &str = r#"<MEET name="meet1" city="Montreal" nation="CAN"><SESSIONS><SESSION number="1" date="2025-01-02"/></SESSIONS></MEET>"#;

    fn from_str(str: &str) -> Result<Meet, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Meet>(str)
    }

    #[test]
    fn builder() {
        let _ = Meet::builder("meet1", "Montreal", "CAN", Vec::new()).build();
    }

    #[test]
    fn required_attributes() {
        from_str(
            r#"<MEET nation="CAN"><SESSIONS><SESSION number="1" date="2025-01-02"></SESSION></SESSIONS></MEET>"#,
        )
        .expect_err("MEET should have a name");

        from_str(
            r#"<MEET name="meet1" city="Montreal"><SESSIONS><SESSION number="1" date="2025-01-02"></SESSION></SESSIONS></MEET>"#,
        )
        .expect_err("MEET should have a nation");

        from_str(r#"<MEET name="meet1" nation="CAN"></MEET>"#)
            .expect_err("MEET should have a city");

        from_str(r#"<MEET name="meet1" city="Montreal" nation="CAN"></MEET>"#)
            .expect_err("MEET should have a SESSIONS");
    }

    #[test]
    fn optional_attributes() {
        let parsed = from_str(MINIMAL).expect("MEET is valid");
        assert_eq!(
            parsed,
            Meet {
                name: "meet1".into(),
                city: "Montreal".into(),
                nation: "CAN".into(),
                sessions: vec![Session {
                    number: 1,
                    date: Date::from_calendar_date(2025, Month::January, 2).unwrap(),
                }]
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());

        const UNKOWN_NATION: &str = r#"<MEET name="meet1" city="Montreal" nation="ZZZ"><SESSIONS><SESSION number="1" date="2025-01-02"/></SESSIONS></MEET>"#;
        let parsed = from_str(UNKOWN_NATION).expect("MEET is valid");
        assert_eq!("ZZZ", parsed.nation);
        assert_eq!(UNKOWN_NATION, to_string(&parsed).unwrap());
    }
}
