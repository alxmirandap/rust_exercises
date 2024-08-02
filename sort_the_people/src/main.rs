#[derive(Debug)]
struct Person {
    name: String,
    height: i32
}

pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut input: Vec<Person> = vec![];
    for (i, name) in names.into_iter().enumerate() {
        input.push(Person {name: name, height: heights[i]});
    }

    input.sort_by(|a, b| b.height.cmp(&a.height));

    return input
        .into_iter()
        .map(|p: Person| p.name)
        .collect();
}

fn main() {
    let names: Vec<String> = vec![
        String::from("Mary"),
        String::from("John"),
        String::from("Emma")
    ];

    let heights: Vec<i32> = vec![180,165,170];

    println!("{:?}", sort_people(names, heights));

}
