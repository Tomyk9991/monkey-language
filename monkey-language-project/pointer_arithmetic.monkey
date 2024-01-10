module monkey-language-project/std.monkey;

let mut r: i32 = 5;
let b = 0;

if (b) {
    r = 20;
} else {
    r = 30;
}

ExitProcess(r);