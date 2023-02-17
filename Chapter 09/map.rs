macro_rules! map{
    ($($key: expr => $value: expr), *) => ({
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
});
}

fn main() {
    let student_grades = map! {
        "alice" => 87,
        "john" => 67,
        "kyle" => 56
    };
    println!("{:?}", student_grades)
}
