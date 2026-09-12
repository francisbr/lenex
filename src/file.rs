use std::io::{Read, Seek, Write};
use std::path::Path;

use crate::error::{Error, Result};
use crate::model::Lenex;

impl Lenex {
    // --- Path convenience: dispatches on file extension (.lef / .lxf, case-insensitive) ---
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let _ = path;
        todo!()
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let _ = path;
        todo!()
    }

    // --- Explicit per-format reader/writer (one method per container, not a format enum —
    //     keeps the LEF path streaming-capable; only LXF needs Seek) ---
    pub fn from_lef_reader(reader: impl Read) -> Result<Self> {
        quick_xml::de::from_reader(std::io::BufReader::new(reader))
            .map_err(|e| Error::Xml(e.to_string()))
    }

    pub fn from_lxf_reader(reader: impl Read + Seek) -> Result<Self> {
        let _ = reader;
        todo!()
    }

    pub fn to_lef_writer(&self, mut writer: impl Write) -> Result<()> {
        let xml = quick_xml::se::to_string(self).map_err(|e| Error::Xml(e.to_string()))?;
        writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
        writer.write_all(xml.as_bytes())?;
        Ok(())
    }

    pub fn to_lxf_writer(&self, writer: impl Write + Seek) -> Result<()> {
        let _ = writer;
        todo!()
    }

    // --- In-memory convenience, shared by both language bindings ---
    pub fn from_lef_bytes(bytes: &[u8]) -> Result<Self> {
        Self::from_lef_reader(bytes)
    }

    pub fn from_lxf_bytes(bytes: &[u8]) -> Result<Self> {
        Self::from_lxf_reader(std::io::Cursor::new(bytes))
    }

    pub fn to_lxf_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.to_lxf_writer(std::io::Cursor::new(&mut buf))?;
        Ok(buf)
    }

    pub fn to_lef_string(&self) -> Result<String> {
        let mut buf = Vec::new();
        self.to_lef_writer(&mut buf)?;
        Ok(String::from_utf8(buf).expect("LEF writer must emit valid UTF-8"))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Lenex;

    #[test]
    fn lef_round_trip() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<LENEX version="3.0">
    <CONSTRUCTOR name="Sauvetage Sportif -- Test Data Generator" registration="Société de Sauvetage" version="1.0">
        <CONTACT email="info@sauvetage.ca"/>
    </CONSTRUCTOR>
</LENEX>
"#;

        let parsed = Lenex::from_lef_bytes(xml.as_bytes()).expect("deserialize");
        let serialized = parsed.to_lef_string().expect("serialize");
        let reparsed = Lenex::from_lef_bytes(serialized.as_bytes()).expect("re-deserialize");

        assert!(serialized.starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
        assert_eq!(parsed, reparsed);
    }
}
