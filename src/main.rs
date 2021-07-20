#[macro_use]
extern crate fdec;

fdec64! {
    module dec,
    name Decimal,
    length 50,
    scale 100
}

use dec::*;
use rayon::prelude::*;

const MAX: usize = 50;

struct Faulhaber {
    comb: Vec<Vec<Decimal>>,
    b: Vec<Decimal>,
}

impl Faulhaber {
    fn new() -> Self {
        let mut comb: Vec<Vec<Decimal>> = vec![vec![];MAX+1];
    comb[0] = vec![Decimal::one()];
    for i in 1..=MAX {
        let mut d = vec![Decimal::zero();i+1];
        d.par_iter_mut()
            .enumerate()
            .for_each(|(j, p)| if j == 0 || j == i { *p = Decimal::one() } else { *p = comb[i-1][j-1] + comb[i-1][j] } );
        comb[i] = d;
    }
    let mut b: Vec<Decimal> = vec![Decimal::one();MAX + 1];
    b[1] = dec!(1) / dec!(2);
    for i in 2..=MAX {
        let sum: Decimal = (0..i).into_par_iter().map(|j| comb[i][j] * b[j] / dec!((i - j + 1) as i64)).reduce(|| Decimal::zero(), |acc, s| acc + s);
        b[i] = Decimal::one() - sum;
    }
        Self {
            comb,
            b,
        }
    }

    fn sum(&self, n: Decimal, p: usize) -> Decimal {
        let mut pow: Vec<Decimal> = vec![Decimal::one();p+2];
        for i in 1..=(p+1) {
            pow[i] = pow[i-1]*n;
        }
        (0..=p).into_par_iter().map(|j| self.comb[p][j] * self.b[j] * pow[p - j + 1] / dec!((p - j + 1) as i64)).reduce(|| Decimal::zero(), |acc, s| acc + s)
    }
}

fn main() {
    let formula = Faulhaber::new();
    for i in 0..=50 {
        println!("{}", formula.sum(Decimal::from(10000), i).trunc());
    }
}
