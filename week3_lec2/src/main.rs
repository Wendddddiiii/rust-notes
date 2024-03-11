#[allow(dead_code)]
fn sum_list(list: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for elem in list {
        sum += *elem; // Dereference elem to access its value
    }
    sum
}

#[allow(dead_code)]
fn first_word(string: &str) -> &str {
    string.split_whitespace().next().unwrap_or("")
}

/// Returns the first half of the list (rounded down).
/// 
/// ```
/// assert_eq!(first_half(&[1, 2, 3, 4]), &[1, 2]);
/// assert_eq!(first_half(&[1, 2, 3, 4, 5]), &[1, 2]);
/// assert_eq!(first_half(&[1, 2, 3, 4, 5, 6]), &[1, 2, 3]);
/// ```
#[allow(dead_code)]
fn first_half(elems: &[i32]) -> &[i32] {
    let halfway = elems.len() / 2;
    &elems[0..halfway]
}

#[cfg(test)]
mod tests {
    use super::{sum_list, first_word};

    #[test]
    fn test_sum_list() {
        assert_eq!(sum_list(&vec![1, 2, 3]), 6);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn test_first_half() {
        assert_eq!(first_half(&[1, 2, 3, 4]), &[1, 2]);
        assert_eq!(first_half(&[1, 2, 3, 4, 5]), &[1, 2]);
        assert_eq!(first_half(&[1, 2, 3, 4, 5, 6]), &[1, 2, 3]);
    }
}

fn main() {

    let s = String::from("hello");
    let borrow: &String = &s;
    
    println!("{}", borrow); // Print borrowed string

}
