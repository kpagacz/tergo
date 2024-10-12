pub(crate) fn trim_line_endings(s: String) -> String {
    s.lines()
        .fold(String::with_capacity(s.len()), |mut acc, line| {
            acc.push_str(line.trim_end());
            acc.push('\n');
            acc
        })
}
