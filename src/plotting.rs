use plotters::prelude::*;
use std::f64;

pub fn draw(xcoords: Vec<f32>) {
    let backend = BitMapBackend::new("chart.png", (2560, 1440)).into_drawing_area();
    let mut ctx = ChartBuilder::on(&backend)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(-4.0..4.0, 0.0..5.5)
        .unwrap();

    backend.fill(&WHITE).unwrap();
    ctx.configure_mesh().draw().unwrap();

    let x_kps: Vec<_> = (-80..80).map(|x| x as f64 / 20.0).collect();
    ctx.draw_series(LineSeries::new(
        x_kps.iter().map(|x| (*x, crate::f(*x as f32) as f64)),
        &RED,
    ))
    .unwrap();

    let y_kps: Vec<_> = (0..120).map(|x| x as f64 / 20.0).collect();
    for x in xcoords.iter() {
        ctx.draw_series(LineSeries::new(
            y_kps.iter().map(|y| (*x as f64, *y)),
            &BLUE,
        ))
        .unwrap();
    }

    backend.present().unwrap();
}
