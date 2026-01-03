struct Person {
    name: *string,
    age: i32,
    height: f32,
    mother: Parent
}

struct Parent {
    name: *string,
    age: i32,
    height: f32
}

let a: *string = "Thomas Kraus";
let b = 5;
let c = 3.14;

let thomas: Person = Person {
    name: a,
    age: 30,
    height: 1.75,
    mother: Parent {
        name: "Irina Kraus",
        age: 60,
        height: 1.65
    }
};

let kekw = 5;