struct Test {
    text: String,
}

fn main() {
    let mut test = Test { text: "Hello".to_string() };
    mutate_test(&mut test);
    println!("Test is {}", test.text);
}

fn mutate_test(test: &mut Test) {
    test.text = "World".to_string();
}
