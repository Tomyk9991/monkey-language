extern fn printf(format: *string, value: i32): void;
extern fn printf(format: *string, value: f64): void;
extern fn printf(format: *string, value: *f64): void;

extern fn ExitProcess(exitCode: i32): void;
extern fn scanf(format: *string, value: *i32): void;