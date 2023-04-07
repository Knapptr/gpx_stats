use std::{env, fs::read_to_string, path::Path};
mod ele;

use xmlparser::Token;
fn main() {
    let doc = read_to_string("daloop.gpx").unwrap();
    let tokens = xmlparser::Tokenizer::from(doc.as_str());
    let mut eles: Vec<f32> = vec![];
    let mut prev: Option<String> = None;
    for token in tokens {
        match token.unwrap() {
            Token::Text { text } => {
                if prev == Some("ele".to_string()) {
                    eles.push(text.parse::<f32>().unwrap());
                }
            }
            Token::ElementStart {
                prefix,
                local,
                span,
            } => {
                prev = Some(local.to_string());
            }
            _ => (),
        }
    }
    let (gain, loss) = ele::get_elevation_change(eles.into_iter());

    println!("Gain: {}\nLoss: {}\nDiff:{}", gain, loss, gain - loss);
}
