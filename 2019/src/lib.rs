pub(crate) fn parse_input<'a, T, F>(s: &'a str, parse_line: F) -> Vec<T>
where
    F: Fn(&'a str) -> T
{
    s
        .lines()
        .map(parse_line)
        .collect()
}

