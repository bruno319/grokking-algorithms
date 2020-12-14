
fn selection_sort<T>(mut list: Vec<T>) -> Vec<T>
    where T: PartialOrd + Copy
{
    let mut sorted_list = Vec::with_capacity(list.len());

    while list.len() > 0 {
        let mut less: (T, usize) = (list[0], 0);
        for x in 1..list.len() {
            let maybe_lesser = list[x];
            if maybe_lesser < less.0 {
                less = (maybe_lesser, x);
            }
        }
        sorted_list.push(less.0);
        list.remove(less.1);
    }

    sorted_list
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn selection_sort_test() {
        let array = vec![101, 4, 83, 7, 21, 5, 9, 99];
        let sorted = selection_sort(array);

        assert_eq!(sorted, vec![4, 5, 7, 9, 21, 83, 99, 101]);

        let array = vec!['c', 'a', 'b', 'e', 'd'];
        let sorted = selection_sort(array);

        assert_eq!(sorted, vec!['a', 'b', 'c', 'd', 'e']);
    }
}