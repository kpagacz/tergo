use parser::expression::Expression;

// Implementing Wadler and https://lindig.github.io/papers/strictly-pretty-2000.pdf

enum Doc {
    Nil,
    Cons(Box<Doc>, Box<Doc>),
    Text(String),
    Nest(usize, Box<Doc>),
    Break,
    Group(Vec<Doc>),
}

enum SimpleDoc {
    Nil,
    Text(String, Box<SimpleDoc>),
    Line(usize, Box<SimpleDoc>),
}

fn simple_doc_to_string(doc: SimpleDoc) -> String {
    match SimpleDoc {
        Nil => "".to_string(),
        Text(s, doc) => format!("{s} {}", simple_doc_to_string(doc)),
        Line(indent, doc) => format!(
            "\n{:width$}{}",
            "",
            simple_doc_to_string(doc),
            width = indent
        ),
    }
}
