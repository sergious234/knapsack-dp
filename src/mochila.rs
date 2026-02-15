use std::cmp::max;

struct Mochila {
    m: usize,
    peso: Box<[usize]>,
    beneficio: Box<[usize]>,
}

impl Mochila {
    pub fn solve(&self) {
        // let mut solucion: [[usize; 11]; 3] = core::array::from_fn(|_| [0;11]);
        let n = self.beneficio.len();

        let mut solucion =
            vec![vec![0; self.m].into_boxed_slice(); n].into_boxed_slice();

        for i in 0..n {
            for w in 0..self.m {
                let value = if i == 0 || w == 0 {
                    0
                } else if self.peso[i] > w {
                    solucion[i - 1][w]
                } else if self.peso[i] <= w {
                    max(
                        solucion[i - 1][w],
                        self.beneficio[i] + solucion[i - 1][w - self.peso[i]],
                    )
                } else {
                    0
                };

                solucion[i][w] = value;
            }
        }

        for i in 0..self.m {
            print!("{:2} ", i + 1);
        }
        println!();
        for row in solucion.iter() {
            for value in row {
                print!("{value:2} ");
            }
            println!()
        }
    }
}
