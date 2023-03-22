pub fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum: Option<u32> = Some(0);
    for &n in nums {
        sum = sum?.checked_add(n);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32() {
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&nums), Some(15));
    }

    #[test]
    fn test_sum_u32_overflow() {
        let nums = [u32::MAX, 1];
        assert_eq!(sum_u32(&nums), None);
    }
}
