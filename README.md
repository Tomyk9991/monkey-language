# Monkey language
A compiled, minimal self written programming language for learning purposes
## The language looks like this:
```
if (hallo) {
    if_stack_variable = 5;

    if(if_stack_variable) {
        nested_if_stack_variable = 13;
    }else{nested_else_stack_variable = "nice";}
} else {
    else_stack_variable = "hallo";
}

if (lello) {
    if_stack_variable = 5;

    if(if_stack_variable) {
        nested_if_stack_variable = 13;
    }
    else{
        nested_else_stack_variable = "hallo";
    }
}
else
{

}

variable = ((4 - 2 * 3 + 1) * -sqrt(3*3+4*4)) / 2;
variable = ((true & true | true | true) & sqrt(false&true&false|true)) & false;

fn hallo(): void {

}

objectVariable =
{
    guten: "Hallo",
    ciau: 5,
    rofl: name(),
    mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
};

nestedObjects = {
    guten: "Hallo",
    ciau: 5,
    mofl: {
        guten: "Hallo",
        ciau: 5,
        property1: name(),
        property2: name(nestedMethod("Hallo", moin("Ciao", 5)))
    },
    rofl: name(),
};

inline = { test: "Hallo", nested: { integer: -51 } };
variable_inside = { test: "Hallo" };

myString = "Strings are great!";
guten_tag = name();
guten_tag = name("Guten Morgen", 5);
name(nestedMethod("Hallo", moin("Ciao", 5)));
hallo = "Github"; tschuess = 5;
mallo = "";
variable_with_another_variable_assignment = hallo;

fn method_name(variable, variable): void {
    function_variable_one = 10;
}

fn f(variable, variable): void
{
    function_variable_two = 10;
}

hallo = "Clion"; ciao = 5;
mallo = "";
```

## Compiling
Compiling is as easy as you think

`cargo build`

## Requirements
For compiling and linking this language uses `nasm` and `ld`. Both is needed in order to use this language.
## Running

`cargo run`

## Arguments
 - `Input file`: 
   - The main source file
 - `wsl`:
   - Currently the language targets a linux x64 environment. Running this in windows requires a subsystem like `wsl`

## Example
### Windows
`cargo run -- --input monkey-language-project/main.monkey --wsl`

### Linux
`cargo run -- --input monkey-language-project/main.monkey`