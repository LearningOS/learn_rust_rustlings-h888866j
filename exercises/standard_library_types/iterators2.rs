// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            // let r = c.collect::<Vec<char>>();
            // r.join("")
            // let x = input.to_string();
            // let x:Vec<char> = x.chars().collect();
            // x[0] = first.to_uppercase().next().unwrap();
            // x.concat()
            let mut ret = "".to_string();
            // ret += &first.to_uppercase().next().unwrap().to_string();
            for ff in first.to_uppercase(){
                ret += &ff.to_string();
            }
            for i in c{
                ret += &i.to_string();
            }
            ret
            

        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word|capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // words.iter()
    // .map(|word|capitalize_first(word))
    // .collect()

    // or 

    words.iter()
    .map(|word|capitalize_first(word))
    .collect::<Vec<String>>()
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
