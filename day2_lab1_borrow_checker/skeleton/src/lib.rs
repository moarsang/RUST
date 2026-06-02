pub fn push_after_reading(values: &mut Vec<i32>) -> Option<i32> {
    // TODO: read the first value, then push first + 10 into the vector.
    // Avoid holding a reference while mutating the vector.

    let first = values.first().copied()?;
    values.push(first + 10);

    Some(first)

}

pub fn sum(values: &[i32]) -> i32 { // 값을 참조해서 새로운 값으로 반환 
    // TODO: this helper should not take ownership of the vector.
    values.iter().sum()
}

pub fn increment_all(values: &mut [i32]) { // 값을 참조해서 해당 값을 수정
    // TODO: increment every value by 1.

    for value in values {
        *value += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_before_push() {
        let mut values = vec![1, 2, 3];
        assert_eq!(push_after_reading(&mut values), Some(1));
        assert_eq!(values, vec![1, 2, 3, 11]);
    }

    #[test]
    fn sums_without_consuming() {
        let values = vec![4, 5, 6];
        assert_eq!(sum(&values), 15);
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn increments_slice() {
        let mut values = vec![1, 2, 3];
        increment_all(&mut values);
        assert_eq!(values, vec![2, 3, 4]);
    }
}
