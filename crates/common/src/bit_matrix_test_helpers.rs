use crate::BitMatrix;


pub fn matrix_to_string(result: &BitMatrix) -> String {
    assert_eq!(1, result.getHeight());
    let mut builder = String::with_capacity(result.getWidth().try_into().unwrap());
    for i in 0..result.getWidth() {
        // for (int i = 0; i < result.getWidth(); i++) {
        builder.push(if result.get(i, 0) { '1' } else { '0' });
    }
    builder
}