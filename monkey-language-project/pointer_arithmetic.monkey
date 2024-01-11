module monkey-language-project/std.monkey;

let format: *string = "%d";
let nice_format: *string = "Die eingegebene Zahl lautet: %d\n";

let size: i32 = 0;
printf("Enter size: ", 0);
scanf(format, &size);

printf(nice_format, size);
printf("Der String ist: %s", "Hallo");

ExitProcess(0);