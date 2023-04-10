use plotters::prelude::*;
use std::{env, fs::read_to_string, ops::Range, path::Path};
mod ele;
mod parse;

use xmlparser::Token;
fn main() {
    parse::parse_it();
}
// fn main() {
//     // probably move to CLI
//     let file_name = "daloop.gpx";
//     let smoothing_amount = 100;
//     // let doc = read_to_string("daloop.gpx").unwrap();
//     // let doc = read_to_string("Formatted_sd.gpx").unwrap();
//     let doc = read_to_string(file_name).unwrap();
//     let tokens = xmlparser::Tokenizer::from(doc.as_str());
//     let mut eles: Vec<f32> = vec![];
//     let mut min: f32 = f32::MAX;
//     let mut max: f32 = 0.0;
//     let mut prev: Option<String> = None;
//     for token in tokens {
//         match token.unwrap() {
//             Token::Text { text } => {
//                 if prev == Some("ele".to_string()) {
//                     let content = text.trim();
//                     if content.len() > 0 {
//                         let parsed_ele = content.parse::<f32>().unwrap();
//                         if parsed_ele > max {
//                             max = parsed_ele;
//                         };
//                         if parsed_ele < min {
//                             min = parsed_ele;
//                         }
//                         eles.push(parsed_ele);
//                     }
//                 }
//             }
//             Token::ElementStart {
//                 prefix,
//                 local,
//                 span,
//             } => {
//                 prev = Some(local.to_string());
//             }
//             _ => (),
//         }
//     }
//     let original_data = eles.clone();
//     // for ele in original_data.iter() {
//     //     println!("ELE: {}", ele);
//     // }
//     let smoothed_data = ele::smooth_ele_data(eles, smoothing_amount);
//     let sample_count = smoothed_data.len();
//     let (s_gain, s_loss) = ele::get_elevation_change(&smoothed_data);
//     let (r_gain, r_loss) = ele::get_elevation_change(&original_data);
//     println!("Samples: {sample_count}");

//     println!(
//         "Raw: Gain: {}\nLoss: {}\nDiff:{}\nmax: {}, min:{}",
//         r_gain,
//         r_loss,
//         r_gain - r_loss,
//         max,
//         min
//     );
//     println!(
//         "Smoothed: Gain: {}\nLoss: {}\nDiff:{}",
//         s_gain,
//         s_loss,
//         s_gain - s_loss
//     );

//     // Plotting
//     let upper_padding = 10.0;
//     let lower_padding = 10.0;
//     let root_area = plotters::prelude::SVGBackend::new("1.5.svg", (1028, 720)).into_drawing_area();
//     root_area.fill(&WHITE).unwrap();

//     let mut ctx = ChartBuilder::on(&root_area)
//         .set_label_area_size(LabelAreaPosition::Left, 40)
//         .set_label_area_size(LabelAreaPosition::Bottom, 40)
//         .caption("Elevation Data", ("sans-serif", 40))
//         .build_cartesian_2d::<Range<usize>, Range<f32>>(
//             0..sample_count,
//             min - lower_padding..max + upper_padding,
//         )
//         .unwrap();

//     ctx.configure_mesh().draw().unwrap();

//     ctx.draw_series(LineSeries::new(
//         smoothed_data.iter().enumerate().map(|(x, y)| (x, *y)),
//         &RED,
//     ));
//     ctx.draw_series(LineSeries::new(
//         original_data.iter().enumerate().map(|(x, y)| (x, *y)),
//         &BLUE,
//     ))
//     .unwrap();
// }
