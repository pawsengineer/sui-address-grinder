pub trait Floorer {
    fn reduce_to_significant_digit(self: Self) -> u64;
}

impl Floorer for u64 {
    fn reduce_to_significant_digit(self: Self) -> u64 {
        self / 1000 * 1000
    }
}
