struct Person {
    name: String,
    friends: Vec<usize>,
}

fn new_person(people: &mut Vec<Person>, name: &str) -> usize {
    people.push(Person {
        name: name.into(),
        friends: Vec::new(),
    });
    people.len() - 1
}

fn add_friend(
    people: &mut Vec<Person>,
    this_id: usize,
    other_id: usize,
) {
    if people[other_id].name != people[this_id].name {
        people[this_id].friends.push(other_id);
    }
}

fn main() {
    let mut people = Vec::new();
    let alice_id = new_person(&mut people, "Alice");
    let bob_id = new_person(&mut people, "Bob");
    add_friend(&mut people, alice_id, bob_id);
    add_friend(&mut people, bob_id, alice_id);
    add_friend(&mut people, alice_id, alice_id); // no-op
}
