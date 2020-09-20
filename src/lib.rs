use num::{Complex, Zero};

pub struct ImageBuffer {
    pub rows: usize,
    pub cols: usize,
    pub buf: Vec<u8>,
}

impl ImageBuffer {
    pub fn new(rows: usize, cols: usize) -> ImageBuffer {
        ImageBuffer {
            rows,
            cols,
            buf: vec![0; rows * cols],
        }
    }
}

/// Given the row and column of a pixel in the output image, return the corresponding point on the
/// complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.  `pixel` is a (column,
/// row) pair indicating a particular pixel in that image.  The `upper_left` and `lower_right`
/// parameters are points on the complex plane designating the area our image covers.
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        // Why subtraction here?  pixel.1 increases as we go down,
        // // but the imaginary component increases as we go up.
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte.  The `upper_left` and `lower_right` arguments specify
/// points on the complex plane corresponding to the upper-left and lower-right corners of the
/// pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of iterations it took for
/// `c` to leave the circle of radius two centered on the origin.  If `c` seems to be a member
/// (more precisely, if we reached the iteration limit without being able to prove that `c` is not
/// a member), return `None`.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex::<f64>::zero();
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

pub fn write_buffer(bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> ImageBuffer {
    let mut pixels = ImageBuffer::new(bounds.0, bounds.1);

    let threads = num_cpus::get();
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.buf.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|s| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                s.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
            .unwrap();
    }
    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 100),
                (25, 75),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 },
            ),
            Complex { re: -0.5, im: -0.5 }
        );
    }
}