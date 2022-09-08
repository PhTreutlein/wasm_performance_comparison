use std::fs;
use std::time::Instant;

fn main() {
    // put name of the file that u want to read here
    let filename = "input_100.txt";

    // reads the file into a string
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // split the file contents to differentiate the numbers
    let converted_contents: Vec<&str> = contents.split(" ").collect();

    // vector for the parsed file contents
    let mut number_vec: Vec<i32> = Vec::new();

    // parse the contents to integers and add the results to number_vec
    for i in 0..converted_contents.len() {
        number_vec.push(
            str::parse::<i32>(&converted_contents[i])
                .expect("Failed to execute: The file contains non-numbers."),
        );
    }

    // start measuring time
    let start = Instant::now();

    // call the sorting function
    let _result = insertion_sort(number_vec);

    // stop the time measuring
    let duration = start.elapsed();

    println!("Time: {:?}", duration);
}

// implementation of the insertion sort algorithm
fn insertion_sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let element = array[i];
        let mut j = i;
        while (j > 0) && (array[j - 1] > element) {
            array[j] = array[j - 1];
            j = j - 1;
        }
        array[j] = element;
    }
    return array;
}

// test module for the sorting function
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn insertion_sort_test() {
        let input = vec!(
            5040, 9139, 9301, 8933, 5997, 5721, 841, 1737, 8151, 7125, 8699, 7161, 9031, 1548,
            8656, 8213, 2260, 817, 2241, 3324, 7027, 1181, 2300
        );
        let expected_result = vec!(
            817, 841, 1181, 1548, 1737, 2241, 2260, 2300, 3324, 5040, 5721, 5997, 7027, 7125, 7161,
            8151, 8213, 8656, 8699, 8933, 9031, 9139, 9301
        );
        let output = insertion_sort(input);
        assert_eq!(output, expected_result);
    }
}
