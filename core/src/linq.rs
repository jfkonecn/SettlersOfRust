use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;

fn group_by<T, K: Eq + std::hash::Hash>(get_key: fn(&T) -> K, list: &[T]) -> HashMap<K, Vec<T>> {
    let mut acc = HashMap::new();
    for item in list {
        let key = get_key(item);
        let list_for_key = acc.entry(key).or_insert(vec![]);
        list_for_key.push(item.clone());
    }
    acc
}

fn sort_by<T, U: Ord + Clone>(
    comparer: fn(&U, &U) -> Ordering,
    get_sort_by_prop: fn(&T) -> U,
    list: &[T],
) -> Vec<T> {
    let mut sorted_list = list.to_vec();
    sorted_list.sort_by(|a, b| comparer(&get_sort_by_prop(a), &get_sort_by_prop(b)));
    sorted_list
}

fn sort_by_char<T: Clone>(get_sort_by_prop: fn(&T) -> char, list: &[T]) -> Vec<T> {
    sort_by(char::cmp, get_sort_by_prop, list)
}

fn sort_by_int<T: Clone>(get_sort_by_prop: fn(&T) -> i32, list: &[T]) -> Vec<T> {
    sort_by(i32::cmp, get_sort_by_prop, list)
}

fn sort_by_float<T: Clone>(get_sort_by_prop: fn(&T) -> f32, list: &[T]) -> Vec<T> {
    sort_by(f32::cmp, get_sort_by_prop, list)
}

fn sort_by_2_tuple_float<T: Clone>(get_sort_by_prop: fn(&T) -> (f32, f32), list: &[T]) -> Vec<T> {
    let compare = |a: &(f32, f32), b: &(f32, f32)| {
        let result = f32::cmp(&a.0, &b.0);
        if result == Ordering::Equal {
            f32::cmp(&a.1, &b.1)
        } else {
            result
        }
    };
    sort_by(compare, get_sort_by_prop, list)
}

fn zip<T, U>(list1: &[T], list2: &[U]) -> Vec<(T, U)> {
    list1
        .iter()
        .zip(list2.iter())
        .map(|(&x, &y)| (x, y))
        .collect()
}
