module monkey-language-project/std.monkey;

let a: bool = ((true | true) & false | (true & true)) & ((false | false) || true | (true & false)) & ((true | false) | false && (false | true));
printf("%d", (i32)a);

ExitProcess(0);