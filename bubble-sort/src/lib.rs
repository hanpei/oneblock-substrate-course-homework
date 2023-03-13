pub fn bubble_sort_i32(list: &mut Vec<i32>) -> &mut Vec<i32> {
    let len = list.len();
    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1)
            }
        }
    }
    list
}

pub fn bubble_sort<T: PartialOrd>(list: &mut Vec<T>) -> &mut Vec<T> {
    let len = list.len();
    for i in 0..len - 1 {
        let mut swapped = false;
        for j in 0..len - 1 - i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                swapped = true
            }
        }
        if swapped == false {
            break;
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_bubble_sort_i32() {
        let mut list = vec![9, 1, 5, 2, 32, 6, 7];
        let sorted = bubble_sort_i32(&mut list);
        assert_eq!(sorted, &mut vec![1, 2, 5, 6, 7, 9, 32]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut list = vec![9, 1, 5, 2, 32, 6, 7];
        let sorted = bubble_sort_i32(&mut list);
        assert_eq!(sorted, &mut vec![1, 2, 5, 6, 7, 9, 32]);
    }

    #[test]
    fn test_bubble_sort_char() {
        let mut list = vec!['a', 'c', 'b', 'd', 'f', 'e'];
        let sorted = bubble_sort(&mut list);
        assert_eq!(sorted, &mut vec!['a', 'b', 'c', 'd', 'e', 'f'])
    }
}
