use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum ParseQueryError {
    InvalidFormat,
    EmptyQuery,
    EmptyAttrs,
}

#[derive(Debug, PartialEq)]
pub enum ParseAttrsErr {
    InvalidFormat,
    TooLittle,
}

pub fn parse_attrs(src: &str, pattern: char, need: usize) -> Result<Vec<&str>, ParseAttrsErr> {
    let parts: Vec<&str> = src.splitn(need, pattern).collect();

    if !(src.contains(pattern)) {
        return Err(ParseAttrsErr::InvalidFormat);
    }

    match parts.len().cmp(&need) {
        Ordering::Less => Err(ParseAttrsErr::TooLittle),
        Ordering::Equal => Ok(parts),
        Ordering::Greater => unreachable!(), //splitn may not return more than n parts
    }
}

pub fn parse_query(src: &str) -> Result<(&str, &str), ParseQueryError> {
    //TODO: handle situation with continuation frames
    // What are they, in the name of Odin?
    if !(src.contains('?')) {
        return Err(ParseQueryError::InvalidFormat);
    }
    let mut elements = src.splitn(2, '?');
    match elements.next() {
        Some("") => Err(ParseQueryError::EmptyQuery),
        Some(cmd) => match elements.next() {
                Some("") => Err(ParseQueryError::EmptyAttrs),
                Some(attrs) => Ok((cmd, attrs)),
                None => Err(ParseQueryError::EmptyAttrs),
        },
        None => Err(ParseQueryError::EmptyQuery),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_pattern() {
        assert_eq!(
            parse_query("some?str"),
            Ok(("some", "str"))
        )
    }

    #[test]
    fn not_a_query() {
        assert_eq!(
            parse_query("somestr"),
            Err(ParseQueryError::InvalidFormat)
        )
    }

    #[test]
    fn empty_query() {
        assert_eq!(
            parse_query("?somestr"),
            Err(ParseQueryError::EmptyQuery)
        )
    }

    #[test]
    fn empty_attrs() {
        assert_eq!(
            parse_query("?somestr"),
            Err(ParseQueryError::EmptyQuery)
        )
    }
}