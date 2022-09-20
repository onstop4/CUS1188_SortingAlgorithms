pub fn insertion_sort(slice: &mut [i32]) {
    for i in 1..slice.len() {
        for j in (1..=i).rev() {
            let existing_value = slice[j];
            let existing_value2 = slice[j - 1];
            if existing_value2 > existing_value {
                (slice[j], slice[j - 1]) = (existing_value2, existing_value);
            }
        }
    }
}

pub fn selection_sort(slice: &mut [i32]) {
    for i in 0..slice.len() {
        let mut lowest_j = i;
        let mut lowest = slice[i];
        for j in (i + 1)..slice.len() {
            let new_value = slice[j];
            if new_value < lowest {
                lowest_j = j;
                lowest = new_value;
            }
        }
        (slice[i], slice[lowest_j]) = (lowest, slice[i]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{insertion_sort, selection_sort};

    #[test]
    fn test_insertion_sort() {
        let mut original = [0, 4, 3, 2, 1, 5, 9, 6, 8, 7];
        let sorted = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        insertion_sort(&mut original);
        assert_eq!(original, sorted);
    }

    #[test]
    fn test_selection_sort() {
        let mut original = [0, 4, 3, 2, 1, 5, 9, 6, 8, 7];
        let sorted = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        selection_sort(&mut original);
        assert_eq!(original, sorted);
    }
}
