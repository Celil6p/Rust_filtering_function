struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, element: &i32) -> bool {
        *element >= self.value
    }
}

fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut result = Vec::new();

    for element in collection.iter() {
        if condition.is_match(element) {
            result.push(*element);
        }
    }

    result
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let condition = FilterCondition { value: 5 };
    let filtered_collection = custom_filter(&collection, &condition);
    println!("{:?}", filtered_collection);
}

