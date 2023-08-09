/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/
extern crate plotters;

use std::iter::{Enumerate, Rev};
use std::ops::Range;
use std::str::Chars;
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::prelude::{BLACK, Color, LineSeries, RED, WHITE};

/// Returns a number as a prettified string (e.g. 10000 to 10,000)
pub fn prettify_number(x: usize) -> String {
    let mut s: String = String::new();
    let i_str: String = x.to_string();
    let a: Enumerate<Rev<Chars>> = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}

/// Displays a Vec<f32> on a graph
pub fn graph_data(l : Vec<f32>, file : &str, zoom : Range<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root).margin(10).set_label_area_size(LabelAreaPosition::Left, 50).set_label_area_size(LabelAreaPosition::Bottom, 40).build_cartesian_2d(0.0..(l.len() as f64), zoom)?;
    let drawing_area = chart.plotting_area();
    let path = PathElement::new(l.iter().enumerate().map(|(x, y)| (x as f64, *y as f64)).collect::<Vec<_>>(), &RED, );
    drawing_area.draw(&path)?;
    chart.configure_series_labels().background_style(WHITE.mix(0.8)).border_style(&BLACK).draw()?;
    Ok(())
}