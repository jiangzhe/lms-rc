
pub trait PlusUnit<T> {
    const UNIT: T;
}

impl PlusUnit<i32> for i32 {
    const UNIT: i32 = 0;
}

impl PlusUnit<usize> for usize {
    const UNIT: usize = 0;
}