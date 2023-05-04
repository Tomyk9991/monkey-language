michi =
{
    guten: "Hallo",
    ciau: 5,
    rofl: name(),
    mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
};

nestedMichi = {
    guten: "Hallo",
    ciau: 5,
    mofl: {
        guten: "Hallo",
        ciau: 5,
        rofl: name(),
        mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
    },
    rofl: name(),
};

moin = { test: "Hallo", nested: { integer: -51 } }; variable_inside = { test: "Hallo" };

fisch = "Fische sind wirklich wirklich toll";
guten_tag = name();
guten_tag = name("Guten Morgen", 5);
name(nestedMethod("Hallo", moin("Ciao", 5)));
hallo = "Thomas"; tschuess = 5;
mallo = "";
variable_with_another_variable_assignment = fisch;