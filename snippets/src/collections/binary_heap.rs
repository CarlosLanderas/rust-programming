use std::collections::BinaryHeap;
#[test]
fn binary_peek_pop_test() {
    //Binary heap organizes the greatest values up to the front of the queue
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);

    //Retrieves a Option<&T> reference without removing
    assert_eq!(heap.peek(), Some(&9));

    //Pops the element in Option<T> format
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));
}

#[test]
fn binary_pop_with_let_test() {
    let mut results = vec![];

    let mut heap = BinaryHeap::new();
    heap.push("c");
    heap.push("a");
    heap.push("r");

    while let Some(value) = heap.pop() {
        results.push(value)
    }

    assert_eq!(results, vec!["r", "c", "a"])
}
