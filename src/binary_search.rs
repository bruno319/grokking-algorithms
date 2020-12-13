
fn binary_search<'a, 'b, T>(list: &'a mut [T], item: &'b T) -> Option<&'a T>
    where T: Ord + PartialEq
{
    list.sort();

    let mut min = 0;
    let mut max = list.len() - 1;

    loop {
        let mid = (min + max) / 2;
        let find = list.get(mid).unwrap();

        if find == item {
            return Some(find);
        }
        if item > find {
            min = mid + 1;
        }
        if item < find {
            max = mid - 1;
        }
        if min > max || max < min {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn binary_search_test() {
        let mut ord_list: Vec<usize> = (0..128).collect();

        let result = binary_search(&mut ord_list, &1);
        assert_eq!(result, Some(&1));

        let mut ord_list = vec!['c', 'd', 'a', 'z', 'f', 'm', 'b', 'u', 'o', 'v'];

        let result = binary_search(&mut ord_list, &'z');
        assert_eq!(result, Some(&'z'));

        let result = binary_search(&mut ord_list, &'p');
        assert_eq!(result, None);
    }
}