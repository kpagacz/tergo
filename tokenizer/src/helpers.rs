use nom_locate::LocatedSpan;

pub type CodeSpan<'a> = LocatedSpan<&'a str>;

#[cfg(test)]
pub(crate) fn assert_parse_eq<T: std::fmt::Debug + PartialEq>(
    left: Result<(&str, T), nom::Err<nom::error::Error<&str>>>,
    right: Result<(&str, T), nom::Err<nom::error::Error<&str>>>,
) {
    match (left, right) {
        (Ok((left_span, left_tree)), Ok((right_span, right_tree))) => {
            assert_eq!(((left_span, left_tree)), ((right_span, right_tree)))
        }
        (Err(::nom::Err::Failure(left_error)), Err(::nom::Err::Failure(right_error))) => {
            assert_eq!(left_error, right_error)
        }
        (Err(::nom::Err::Incomplete(_)), _) => unreachable!(),
        (_, Err(::nom::Err::Incomplete(_))) => panic!("We're only using complete strings here!"),
        (l, r) => assert_eq!(l, r),
    }
}
