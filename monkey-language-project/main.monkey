let a = "Hello, World!";
let b = 5;
let c = 3.14;
let d = a;

struct Person {
    name: *string,
    age: i32,
    height: f32
}

let thomas: Person = Person {
    name: a,
    age: 30,
    height: 5.9
};