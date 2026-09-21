use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Builder, Clone)]
#[builder(on(_, into))]
#[serde(rename = "SWIMSTYLE")]
pub struct SwimStyle {
    #[builder(start_fn)]
    #[serde(rename = "@distance")]
    pub distance: u32,

    #[builder(start_fn)]
    #[serde(rename = "@relaycount")]
    pub relaycount: u32,

    #[builder(start_fn)]
    #[serde(rename = "@stroke")]
    pub stroke: String,
}

impl<S: swim_style_builder::IsComplete> From<SwimStyleBuilder<S>> for SwimStyle {
    fn from(b: SwimStyleBuilder<S>) -> SwimStyle {
        b.build()
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;

    use super::SwimStyle;

    const MINIMAL: &str = r#"<SWIMSTYLE distance="25" relaycount="1" stroke="FREE"/>"#;

    fn from_str(str: &str) -> Result<SwimStyle, quick_xml::de::DeError> {
        quick_xml::de::from_str::<SwimStyle>(str)
    }

    #[test]
    fn builder() {
        let _ = SwimStyle::builder(25_u32, 1_u32, "FREE").build();
    }

    #[test]
    fn required_attributes() {
        from_str(r#"<SWIMSTYLE relaycount="1" stroke="FREE"/>"#)
            .expect_err("SWIMSTYLE should have a distance");
        from_str(r#"<SWIMSTYLE distance="25" stroke="FREE"/>"#)
            .expect_err("SWIMSTYLE should have a relaycount");
        from_str(r#"<SWIMSTYLE distance="25" relaycount="1"/>"#)
            .expect_err("SWIMSTYLE should have a stroke");
    }

    #[test]
    fn serialization() {
        let parsed = from_str(MINIMAL).expect("SWIMSTYLE is valid");
        assert_eq!(
            parsed,
            SwimStyle {
                distance: 25,
                relaycount: 1,
                stroke: "FREE".to_string(),
            }
        );
        assert_eq!(MINIMAL, to_string(&parsed).unwrap());
    }
}
