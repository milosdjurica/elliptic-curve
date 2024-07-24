use num_bigint::BigUint;

#[derive(Clone)]
pub struct Point {
    x: Option<BigUint>,
    y: Option<BigUint>,
}

impl Point {
    pub fn new(x: Option<BigUint>, y: Option<BigUint>) -> Self {
        Point { x, y }
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}
