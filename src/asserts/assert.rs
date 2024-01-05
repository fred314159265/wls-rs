use num_traits::float::FloatCore;

#[cfg(test)]
pub fn assert_almost_equal<T: FloatCore>(expected: T, actual: T, delta: T) {
    assert!(delta > expected - actual);
}

#[allow(dead_code)]
pub fn assert_true(condition: bool) {
    assert!(condition)
}

pub fn assert_have_same_size<T: FloatCore>(items_1: &[T], items_2: &[T]) {
    assert_eq!(items_1.len(), items_2.len())
}

pub fn assert_have_size_greater_than_two<T: FloatCore>(items: &[T]) {
    assert!(items.len() >= 2)
}
