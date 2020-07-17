#[derive(Debug, PartialEq)]
pub enum ParseQueryError {
   EmptyQuery,
   EmptyAttrs,
}

pub fn parse_query(src: &str) -> Result<(&str, &str), ParseQueryError> {
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
            Err(ParseQueryError::EmptyAttrs)
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
