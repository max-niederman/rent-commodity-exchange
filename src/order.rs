use std::ops::Range;

#[derive(Debug, Clone)]
pub enum Order {
    Rect {
        price: f64,
        size: i32,
        time: Range<usize>,
    },
}

impl Order {
    /// Price of the order.
    pub fn price(&self) -> f64 {
        match self {
            Order::Rect { price, .. } => *price,
        }
    }

    /// Shape of the order.
    pub fn shape(&self, t_max: usize) -> Vec<i32> {
        match self {
            Order::Rect { size, time, .. } => {
                let mut shape = vec![0; t_max];
                shape[time.clone()].fill(*size);
                shape
            }
        }
    }

    /// Quote the fulfillment of an order.
    /// I.e., how much money do we get by fulfilling the order?
    pub fn quote_fulfillment(&self) -> f64 {
        match self {
            Order::Rect { price, size, .. } => price * *size as f64,
        }
    }

    #[cfg(feature = "rand")]
    pub fn random_rect(rng: &mut impl rand::Rng, t_max: usize) -> Order {
        let price = 1.0 + 0.2 * rng.random::<f64>();
        let size = rng.random_range(-10..=10);
        let start = rng.random_range(0..t_max);
        let end = rng.random_range(start..t_max);

        Order::Rect {
            price,
            size,
            time: start..end,
        }
    }
}
