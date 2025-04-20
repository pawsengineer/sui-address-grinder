pub trait Floorer {
    fn reduce_to_significant_digit(self: Self) -> u64;
}

impl Floorer for u64 {
    fn reduce_to_significant_digit(mut self: Self) -> u64 {
        let mut magnitude = 1;
        while self >= 10 {
            self /= 10;
            magnitude *= 10;
        }
        self * magnitude
    }
}
