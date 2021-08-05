use ndarray::{Array1, Array2, Array};
use ndarray_linalg::least_squares::LeastSquaresSvd;

const fn chebyshev(x: f32, n: usize) -> f32 {
    match n {
        0 => 1.0,
        1 => x,
        2..=20 => 2.0 * x * chebyshev(x, n-1) - chebyshev(x, n-2),
        _ => 0.0
    }
}

pub struct Chebyshev {
    range: (f32, f32),
    halfwidth: f32,
    coefficients: Array1<f32>,
}

impl Chebyshev {
    pub fn new(data: (Array1<f32>, Array1<f32>), degree: usize) -> Self {
        let n = data.0.len();
        let (start, end) = (data.0[0], data.0[n-1]);
        let halfwidth = (end - start) / 2.0;

        let chebys = data.0
            .mapv(|x| (x - start - halfwidth) / halfwidth)
            .into_iter()
            .map(|x| (0..degree).map(move |deg| chebyshev(x, deg)))
            .flatten();

        let chebys = Array::from_iter(chebys).into_shape((n, degree)).unwrap();
        let beta = chebys.least_squares(&data.1).unwrap();

        Chebyshev {
            range: (start, end),
            halfwidth,
            coefficients: beta.solution
        }
    }

    pub fn eval(&self, x: f32) {
        let x = (x - self.range[0] - self.halfwidth) / self.halfwidth;
        let mut b2 = self.coefficients[self.coefficients.len()-1];
        let mut b1 = 2. * x * b2 + self.coefficients[self.coefficients.len()-2];

        let mut n = self.coefficients.len();
        while n > 2 {
            let tmp = 2. * x * b1 + self.coefficients[self.coefficients.len() - 1 - n] - b2;
            b2 = b1;
            b1 = tmp;
        }
        x * b1 + self.coefficients[0] - b2
    }
}
