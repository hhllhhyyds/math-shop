use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    style::{colors::colormaps::MandelbrotHSL, BLACK, WHITE},
};
use std::ops::Range;

use refined_float::{Complex, Float64};

const OUT_FILE_NAME: &str = "target/mandelbrot.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (xr, yr) = (chart.x_range(), chart.y_range());

    for (z, c) in mandelbrot_set(xr, yr, (pw as usize, ph as usize), 100) {
        if c != 100 {
            plotting_area.draw_pixel(
                (z.real.0, z.imag.0),
                &MandelbrotHSL::get_color(c as f64 / 100.0),
            )?;
        } else {
            plotting_area.draw_pixel((z.real.0, z.imag.0), &BLACK)?;
        }
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn mandelbrot_set(
    real: Range<f64>,
    complex: Range<f64>,
    samples: (usize, usize),
    max_iter: usize,
) -> impl Iterator<Item = (Complex<Float64>, usize)> {
    let step = (
        Float64((real.end - real.start) / samples.0 as f64),
        Float64((complex.end - complex.start) / samples.1 as f64),
    );
    (0..(samples.0 * samples.1)).map(move |k| {
        let k_2d = (
            Float64((k % samples.0) as f64),
            Float64((k / samples.0) as f64),
        );
        let c = Complex::new(Float64(real.start), Float64(complex.start))
            + Complex::new(step.0 * k_2d.0, step.1 * k_2d.1);
        let mut z = Complex::new(Float64::ZERO, Float64::ZERO);
        let mut cnt = 0;
        while cnt < max_iter && z.length() <= Float64(1e10) {
            z = z * z + c;
            cnt += 1;
        }
        (c, cnt)
    })
}
