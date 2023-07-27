
struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        *item == self.value
    }
}

fn custom_filter<T>(collection: &[T], filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut filtered_items = Vec::new();
    for item in collection {
        if filter_condition.is_match(item) {
            filtered_items.push(item.clone());
        }
    }
    filtered_items
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter_condition = FilterCondition { value: 5 };

    let filtered_result = custom_filter(&collection, &filter_condition);

    println!("Filtered result: {:?}", filtered_result);
}
