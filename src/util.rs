use rand::Rng;

/// Returns a random element from a slice.
///
/// # Panics
/// Panics if the slice is empty.
pub fn get_random_element<T>(vector: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..vector.len());
    &vector[random_index]
}
