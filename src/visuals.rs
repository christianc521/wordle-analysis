use full_palette::BLACK;
use plotters::prelude::*;
use std::{collections::HashMap, path::Path};

pub fn draw_bigram_bar(
    frequencies: HashMap<String, i32>,
    caption: &str,
    filename: String,
    x_limit: Option<usize>,
) {
    // Convert to a vector and sort by frequency (decending)
    let mut data: Vec<_> = frequencies.into_iter().collect();
    data.sort_by(|a, b| b.1.cmp(&a.1));

    // Limit to the top 20 bigrams
    let mut top_data: Vec<(String, i32)> = Vec::new();
    match x_limit {
        Some(limit) => {
            if data.len() > limit {
                top_data = data[0..limit].iter().cloned().collect()
            }
        }
        None => top_data = data.clone(),
    };

    // Get the y-axis limit
    let max_frequency = top_data
        .iter()
        .map(|(_, c)| *c as i32)
        .max()
        .expect("Failed to find max frequency.");

    let path_string = format!("data-visuals/{}.png", filename);
    let path = Path::new(&path_string);
    let drawing_area = BitMapBackend::new(path, (1024, 768)).into_drawing_area();

    drawing_area
        .fill(&WHITE)
        .expect("Failed to clear drawing area.");

    let labels: Vec<_> = top_data
        .clone()
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<_>>();

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .margin(20)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(labels.into_segmented(), 0..max_frequency + 10)
        .expect("Failed to build chart.");

    let label_style = TextStyle::from(("sans-serif", 20).into_font()).color(&(BLACK));
    chart
        .configure_mesh()
        .x_labels(max_frequency as usize)
        .label_style(label_style)
        .draw()
        .expect("Failed to draw bigram chart.");

    let _ = chart.draw_series(
        Histogram::vertical(&chart)
            .margin(10)
            .style(BLUE.filled())
            .data(top_data.iter().clone().map(|x| (&x.0, x.1))),
    );
}

pub fn draw_vowel_pie(
    data: &[f64],
    colors: &[RGBColor],
    title: String,
    filename: String,
    labels: &[String],
) {
    let path_string = format!("data-visuals/{}.png", filename);
    let path = Path::new(&path_string);
    let drawing_area = BitMapBackend::new(path, (700, 500)).into_drawing_area();
    let _ = drawing_area
        .fill(&WHITE)
        .expect("Failed to clear pie chart drawing area.");

    let title_style = TextStyle::from(("sans-serif", 40).into_font()).color(&(BLACK));
    let label_style = TextStyle::from(("sans-serif", 20).into_font()).color(&(BLACK));
    let percent_style = TextStyle::from(("sans-serif", 20).into_font()).color(&(WHITE));
    let _ = drawing_area.titled(&title, title_style);

    let mut pie = Pie::new(&(350, 250), &150.0, data, colors, labels);
    pie.start_angle(-90.0);
    pie.label_style(label_style);
    pie.percentages(percent_style);
    pie.label_offset(10.0);

    drawing_area.draw(&pie).expect("Failed to draw pie chart.");
}
