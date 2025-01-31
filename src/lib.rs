#![feature(test)]

extern crate test;

pub use order::Order;
use russcip::{ProblemCreated, prelude::*};

pub mod order;

#[derive(Debug)]
pub struct Problem {
    fulfillments: Vec<russcip::Variable>,
    model: Model<ProblemCreated>,
}

impl Problem {
    pub fn new(orders: &[Order], t_max: usize) -> Self {
        let mut model = Model::default().maximize();

        if cfg!(not(debug_assertions)) {
            model = model.hide_output();
        }

        // Add a variable for each order.
        let mut fulfillments = vec![];
        for (i, order) in orders.iter().enumerate() {
            fulfillments.push(model.add(var().binary().obj(order.quote_fulfillment())));
        }

        // Column-major matrix mapping order fulfillments to capacity requirements.
        let capacity_mat = orders
            .iter()
            .map(|order| order.shape(t_max))
            .collect::<Vec<_>>();

        // Add the capacity constraints.
        for t in 0..t_max {
            let mut constraint = cons();

            for (i, shape) in capacity_mat.iter().enumerate() {
                constraint = constraint.coef(&fulfillments[i], shape[t] as f64);
            }

            model.add(constraint.le(0.0));
        }

        Self {
            fulfillments,
            model,
        }
    }

    /// Solve the problem to get the (surplus, fulfillment) tuple.
    pub fn solve(self) -> (f64, Vec<bool>) {
        let solution = self.model.solve().best_sol().expect("no solution found");

        (
            solution.obj_val(),
            self.fulfillments
                .iter()
                .map(|var| solution.val(var) > 0.5)
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_single_trade() {
        let orders = vec![
            Order::Rect {
                price: 2.0,
                size: 1,
                time: 0..1,
            },
            Order::Rect {
                price: 1.0,
                size: -1,
                time: 0..1,
            },
        ];
        let problem = Problem::new(&orders, 1);
        let (surplus, fulfillments) = problem.solve();

        assert_eq!(surplus, 1.0);
        assert_eq!(fulfillments, [true, true]);
    }

    fn random_orders(n: usize, t_max: usize) -> Vec<Order> {
        let mut rng = rand::rng();
        (0..n)
            .map(|_| Order::random_rect(&mut rng, t_max))
            .collect()
    }

    #[bench]
    fn bench_problem_construction(b: &mut test::Bencher) {
        let t_max = 24 * 7;

        b.iter(|| Problem::new(&random_orders(1000, t_max), t_max));
    }

    #[bench]
    fn bench_problem_solve(b: &mut test::Bencher) {
        let t_max = 24 * 7;

        b.iter(|| Problem::new(&random_orders(1000, t_max), t_max).solve());
    }
}
