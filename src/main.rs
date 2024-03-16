#[derive(Debug)]
struct Name<'a, 'b> {
    first: &'a str,
    last: &'b str,
}

impl<'a> Name<'a, 'a> {
    pub fn new(name: &'a str) -> Self {
        let fields: Vec<&str> = name.split_whitespace().collect();
        assert_eq!(2, fields.len());
        Name { first: fields[0], last: fields[1] }
    }
}

impl<'a, 'b> Name<'a, 'b> {
    pub fn new_split(first: &'a str, last: &'b str) -> Self {
        Name { first, last }
    }
}

fn main() {
    let s = "Bart Massey".to_string();
    println!("{:?}", Name::new(&s));
    println!("{:?}", Name::new_split("Bart", "Massey"));
}
