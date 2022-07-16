// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)


use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError)
}

impl ParsePosNonzeroError {
    // TODO: add another error conversion function here.
    fn from_creation(e: CreationError) -> ParsePosNonzeroError{
        ParsePosNonzeroError::Creation(e)
    }
    fn from_parse(e: ParseIntError) -> ParsePosNonzeroError{
        ParsePosNonzeroError::ParseInt(e)
    }
}

fn parse_pos_nonzero(s: &str)
    -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
{
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.

    let x:i64 = s.parse()
        .map_err(|e| ParsePosNonzeroError::ParseInt(e))?;

    // the whole `.map_err(xxx)?;` is equivalent to the following `match`
    let x = match s.parse::<i64>(){
        Ok(v) => v,
        Err(e) => return  Err(ParsePosNonzeroError::ParseInt(e))
    };

    // `.map_err(xxx)` method is equivalent to the following `match`
    let x: i64 = match s.parse::<i64>(){
        Ok(v) => Ok(v), // leave untouched
        Err(e) => Err(ParsePosNonzeroError::ParseInt(e))
    }?; // usage of `?`.

    // map_err 里自然也可以不用闭包，用和下面类似的方式写一个函数(参数要标注类型)传进去, 然后也得接？提取值和抛出错误
    let x = s.parse::<i64>().map_err(ParsePosNonzeroError::from_parse)?;

    // 有没有其他方法可用呢，  `map_or_else` 配合 `?` 也可以，就是麻烦
    let x = s.parse::<i64>()
        .map_or_else( |w|Err(ParsePosNonzeroError::ParseInt(w)),
                      |q|Ok(q))?;

    // 单独的 `?` 也可以用match替换
     let x = match s.parse::<i64>()
        .map_or_else( |w|Err(ParsePosNonzeroError::ParseInt(w)),
                      |q|Ok(q))
     {
         Ok(v) => v,
         Err(e) => return Err(e)
     };

    // Likewise，the `?`after `.map_err(xxx)` is equivalent to the following `match`
    let x: i64 = match s.parse::<i64>().map_err(|e|ParsePosNonzeroError::ParseInt(e)){
        Ok(v) => v,
        Err(e) => return Err(e) // Err untouched, just return it
    };

    PositiveNonzeroInteger::new(x)
        .map_err(ParsePosNonzeroError::from_creation)

    // match PositiveNonzeroInteger::new(x){
    //     Ok(r) => Ok(r),
    //     Err(t) => Err(ParsePosNonzeroError::Creation(t))
    // }
}


// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
