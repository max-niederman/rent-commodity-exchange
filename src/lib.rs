#![feature(test)]

extern crate test;

pub use order::Order;

pub mod order;

pub struct Problem {
    fulfillments: Vec<microlp::Variable>,
    inner: microlp::Problem,
}

impl Problem {
    pub fn new(orders: &[Order], t_max: usize) -> Self {
        let mut inner = microlp::Problem::new(microlp::OptimizationDirection::Maximize);

        // Add a variable for each order.
        let mut fulfillments = vec![];
        for (i, order) in orders.iter().enumerate() {
            fulfillments.push(inner.add_binary_var(order.quote_fulfillment()));
        }

        // Column-major matrix mapping order fulfillments to capacity requirements.
        let capacity_mat = orders
            .iter()
            .map(|order| order.shape(t_max))
            .collect::<Vec<_>>();

        // Add the capacity constraints.
        for t in 0..t_max {
            inner.add_constraint(
                capacity_mat
                    .iter()
                    .enumerate()
                    .map(|(i, shape)| (fulfillments[i], shape[t] as f64)),
                microlp::ComparisonOp::Le,
                0.0,
            )
        }

        Self {
            fulfillments,
            inner,
        }
    }

    /// Solve the problem to get the (surplus, fulfillment) tuple.
    pub fn solve(&self) -> (f64, Vec<bool>) {
        let solution = self.inner.solve().unwrap();

        (
            solution.objective(),
            self.fulfillments
                .iter()
                .map(|&var| *solution.var_value(var) > 0.5)
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

        let orders = random_orders(1000, t_max);
        b.iter(|| Problem::new(&orders, t_max));
    }

    #[bench]
    fn bench_problem_solve(b: &mut test::Bencher) {
        let t_max = 24 * 7;

        let orders = random_orders(10, t_max);
        let problem = Problem::new(&orders, t_max);
        b.iter(|| problem.solve());
    }
}
