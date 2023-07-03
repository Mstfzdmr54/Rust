struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, element: &i32) -> bool {
        *element > self.value
    }
}

fn custom_filter<T>(collection: &[T], condition: &FilterCondition) -> Vec<T>
where
    T: std::clone::Clone + std::cmp::PartialOrd,
{
    collection
        .iter()
        .filter(|element| condition.is_match(element))
        .cloned()
        .collect()
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5];
    let condition = FilterCondition { value: 3 };

    let filtered_result = custom_filter(&collection, &condition);

    println!("Filtered Result: {:?}", filtered_result);
}
