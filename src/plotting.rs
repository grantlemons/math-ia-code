use plotters::prelude::*;
use std::f64;

pub fn draw(xcoords: Vec<f32>) {
    let backend = BitMapBackend::new("chart.png", (1920, 1080)).into_drawing_area();
    let mut ctx = ChartBuilder::on(&backend)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Normal Distribution", ("sans-serif", 40))
        .build_cartesian_2d(-4.0..4.0, 0.0..5.5)
        .unwrap();

    backend.fill(&WHITE).unwrap();
    ctx.configure_mesh().draw().unwrap();

    let x_kps: Vec<_> = (-80..80).map(|x| x as f64 / 20.0).collect();
    ctx.draw_series(LineSeries::new(
        x_kps.iter().map(|x| {
            (
                *x,
                (13_f64 / f64::sqrt(2_f64 * f64::consts::PI))
                    * f64::powf(f64::consts::E, -(1_f64 / 2_f64) * f64::powi(*x, 2)),
            )
        }),
        &RED,
    ))
    .unwrap()
    .label("Normal Distribution Function");

    let y_kps: Vec<_> = (0..120).map(|x| x as f64 / 20.0).collect();
    for x in xcoords.iter() {
        ctx.draw_series(LineSeries::new(
            y_kps.iter().map(|y| (*x as f64, *y)),
            &BLUE,
        ));
    }

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
    backend.present();
}
