struct Person {
    name: *string,
    age: i32,
    height: f32,
    mother: Person
}

let a: *string = "Thomas Kraus";

let thomas: Person = Person {
    name: a,
    age: 30,
    height: 5.9,
    mother: Person {
        name: "Irina Kraus",
        age: 60,
        height: 5.5
    }
};