let a = 10 + 2;
if (a) {
    a = 1;

    if (a) {
        a = 20;
    } else {
        a = 13;
    }
} else {
    a = 0;
    if (a) {
        a = 30;
    } else {
        a = 244;
    }
}

exit(a);