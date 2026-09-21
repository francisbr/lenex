use bon::Builder;
use serde::{Deserialize, Serialize};

pub mod swimstyle;

use swimstyle::SwimStyle;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Builder, Clone)]
#[builder(on(_, into))]
#[serde(rename = "EVENT")]
pub struct Event {
    #[builder(start_fn)]
    #[serde(rename = "@eventid")]
    pub eventid: u32,

    #[builder(start_fn)]
    #[serde(rename = "@number")]
    pub number: u32,

    #[builder(start_fn)]
    #[serde(rename = "SWIMSTYLE")]
    pub swimstyle: SwimStyle,
}

impl<S: event_builder::IsComplete> From<EventBuilder<S>> for Event {
    fn from(b: EventBuilder<S>) -> Event {
        b.build()
    }
}

pub mod events_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    type InnerType = super::Event;

    #[derive(Serialize)]
    #[serde(rename = "EVENTS")]
    struct Borrowed<'a> {
        #[serde(rename = "EVENT")]
        inner: &'a [InnerType],
    }

    #[derive(Deserialize)]
    #[serde(rename = "EVENTS")]
    struct Owned {
        #[serde(rename = "EVENT", default)]
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

    use super::{Event, swimstyle::SwimStyle};

    const MINIMAL: &str = r#"<EVENT eventid="1" number="1"><SWIMSTYLE distance="25" relaycount="1" stroke="FREE"/></EVENT>"#;

    fn from_str(str: &str) -> Result<Event, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Event>(str)
    }

    #[test]
    fn builder() {
        let _ = Event::builder(1_u32, 1_u32, SwimStyle::builder(25_u32, 1_u32, "FREE")).build();
    }

    #[test]
    fn required_attributes() {
        from_str(
            r#"<EVENT number="1"><SWIMSTYLE distance="25" relaycount="1" stroke="FREE"/></EVENT>"#,
        )
        .expect_err("EVENT should have an eventid");
        from_str(
            r#"<EVENT eventid="1"><SWIMSTYLE distance="25" relaycount="1" stroke="FREE"/></EVENT>"#,
        )
        .expect_err("EVENT should have a number");
        from_str(r#"<EVENT eventid="1" number="1"/>"#).expect_err("EVENT should have a SWIMSTYLE");
    }

    #[test]
    fn serialization() {
        let parsed = from_str(MINIMAL).expect("EVENT is valid");
        assert_eq!(
            parsed,
            Event {
                eventid: 1,
                number: 1,
                swimstyle: SwimStyle {
                    distance: 25,
                    relaycount: 1,
                    stroke: "FREE".to_string(),
                },
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());
    }
}
