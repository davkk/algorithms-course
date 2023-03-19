struct Matrix {
    data: Vec<Vec<f32>>,
}

impl Matrix {
    fn new(data: Vec<Vec<f32>>) -> Self {
        Self { data }
    }

    fn default(dim: usize) -> Self {
        Self {
            data: vec![vec![0.0; dim]; dim],
        }
    }

    fn get(&self, i: usize, j: usize) -> f32 {
        self.data[i][j]
    }

    fn set(&mut self, i: usize, j: usize, value: f32) {
        self.data[i][j] = value;
    }

    fn multiply(&self, other: &Self) -> Self {
        let mut result = Matrix::default(self.data.len());
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                for k in 0..self.data[i].len() {
                    result.set(
                        i,
                        j,
                        result.get(i, j) + self.get(i, k) * other.get(k, j),
                    );
                }
            }
        }
        result
    }

    fn lu_decompose(&self) -> (Self, Self) {
        let n = self.data.len();

        let a = self;
        let mut l = Self::default(n);
        let mut u = Self::default(n);

        for i in 0..n {
            l.set(i, i, 1.0);

            for k in i..n {
                let sum: f32 = (0..i)
                    .map(|j| l.get(i, j) * u.get(j, k))
                    .sum();

                u.set(i, k, a.get(i, k) - sum);
            }

            for k in i..n {
                let sum: f32 = (0..i)
                    .map(|j| l.get(k, j) * u.get(j, i))
                    .sum();

                l.set(k, i, (a.get(k, i) - sum) / u.get(i, i));
            }
        }

        (l, u)
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().try_for_each(|row| {
            row.iter()
                .try_for_each(|value| write!(f, "{:>7.2} ", value))?;
            write!(f, "\n")
        })
    }
}

fn main() {
    let matrix = Matrix::new(vec![
        vec![9.0, 8.0, -2.0, 2.0, -2.0],
        vec![7.0, -3.0, -2.0, 7.0, 2.0],
        vec![2.0, -2.0, 2.0, -7.0, 6.0],
        vec![4.0, 8.0, -3.0, 3.0, -1.0],
        vec![2.0, 2.0, -1.0, 1.0, 4.0],
    ]);

    println!("A =\n{matrix}");

    let (l, u) = matrix.lu_decompose();
    println!("L =\n{l}");
    println!("U =\n{u}");

    println!("L * U =\n{}", l.multiply(&u));
}
