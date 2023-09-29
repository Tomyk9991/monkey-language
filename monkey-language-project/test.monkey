module monkey-language-project/std.monkey;

let format: string = "The formatted thing is:\n\t%d\n";
let mut value: i32 = 5;
value = value + 1;

let s = 5;

printf(format, value);
printf("%d", value + 1);

ExitProcess(0);