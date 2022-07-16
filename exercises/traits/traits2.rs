// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!



trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here
impl AppendBar for Vec<String>{
    fn append_bar(self) -> Self{
        self.iter().map(|v| v.to_owned() + "Bar").collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo"), "Bar".to_string()].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("BarBar"));
        assert_eq!(foo.pop().unwrap(), String::from("FooBar"));
    }
}
