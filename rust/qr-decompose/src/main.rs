use ndarray::{array, Array1, Array2};

trait Norm {
    fn norm(&self) -> f32;
}

impl Norm for Array1<f32> {
    fn norm(&self) -> f32 {
        self.fold(0.0, |acc, x| acc + x * x)
            .sqrt()
    }
}

trait QRDecompose
where
    Self: Sized,
{
    fn qr_decompose(&self) -> (Self, Self);
}

impl QRDecompose for Array2<f32> {
    fn qr_decompose(&self) -> (Self, Self) {
        println!("{:?}", self);
        let id: Array2<f32> = Array2::eye(self.ncols());

        let mut r = self.clone();
        let mut q = id.clone();

        for i in 0..self.ncols() - 1 {
            let r_col: Array1<f32> = r
                .column(i)
                .map(
                    |x| {
                        if *x < i as f32 {
                            0.0
                        } else {
                            *x
                        }
                    },
                )
                .to_owned();

            let r_col_norm = r_col.norm();

            let v = r_col.into_shape((self.ncols(), 1)).unwrap()
                + r_col_norm
                    * id.column(i)
                        .to_owned()
                        .into_shape((self.ncols(), 1))
                        .unwrap();

            let v_matrix = v.dot(&v.t());
            let v_scalar = v.t().dot(&v).sum();

            let h = &id - 2.0 * v_matrix / v_scalar;

            r = h.dot(&r);
            q = h.dot(&q);
        }

        (q, r)
    }
}

fn main() {
    let matrix: Array2<f32> =
        array![[1.0, 2.0, -1.0], [1.0, 4.0, 5.0], [1.0, 4.0, 1.0]];

    let (q, r) = matrix.qr_decompose();

    r.dot(&q.t()).qr_decompose();
}
