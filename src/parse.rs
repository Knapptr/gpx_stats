pub enum ParseError {
    RoXML(String),
    NAN(String),
    InvalidData(String),
    BadErrorHandling(String),
}
impl From<roxmltree::Error> for ParseError {
    fn from(rx_error: roxmltree::Error) -> Self {
        match rx_error {
            roxmltree::Error::ParserError(e) => Self::RoXML(e.to_string()),
            _ => Self::BadErrorHandling("Error with ROXML".to_string()),
        }
    }
}
impl From<&str> for ParseError {
    fn from(str: &str) -> Self {
        Self::InvalidData(str.to_string())
    }
}
impl From<std::num::ParseFloatError> for ParseError {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::NAN(error.to_string())
    }
}
pub struct GPXData {
    eles: Vec<f32>,
}
pub fn parse_it() -> Result<GPXData, ParseError> {
    let document = std::fs::read_to_string("daloop.gpx").unwrap();
    let parsed_doc = roxmltree::Document::parse(&document)?;
    // println!("{:?}", parsed_doc);
    let mut eles = vec![];
    for element in parsed_doc.descendants() {
        if element.is_element() {
            if element.tag_name().name() == "ele" {
                let ele = element.text().ok_or("Parsing Error")?;
                let parsed = ele.parse::<f32>()?;
                eles.push(parsed);
            }
        }
    }
    Ok(GPXData { eles })
}
