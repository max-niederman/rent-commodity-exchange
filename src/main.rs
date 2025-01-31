use rent_commodity_exchange::{Order, Problem};

fn main() {
    let t_max = 24 * 365;
    let orders = random_orders(500, t_max);

    let problem = Problem::new(&orders, t_max);
    let (surplus, fulfillments) = problem.solve();

    println!("Solved problem with fulfillments {fulfillments:?}");
    println!("and surplus {surplus}.");
}

fn random_orders(n: usize, t_max: usize) -> Vec<Order> {
    let mut rng = rand::rng();
    (0..n)
        .map(|_| Order::random_rect(&mut rng, t_max))
        .collect()
}
