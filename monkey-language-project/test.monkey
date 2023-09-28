module monkey-language-project/std.monkey;

let format: string = "The formatted thing is:\n\t%d\n";
let value: i32 = 5;

printf(format, value);
printf("%d", value + 1);

ExitProcess(value);