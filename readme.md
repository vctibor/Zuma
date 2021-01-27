ZUMA - Vector Graphics Language
===============================

## Datatypes

ZUMA is strongly typed.

Following datatypes can be created using literals:

**Boolean** has one of values `true` or `false`.

**Number** is a single precision floating point, i.e. f32: `1.5464`.

**Point** is declared using two numbers inside square brackets like `[4.45 6.06]`.

**Color** can be declared using sharp followed by hexadecimal value: `#ff001a`. Additionally few basic colors can be declared by their name: `black`, `white`, `red`, `green`, `blue` or `yellow`.

## Expressions

Expressions are delimited using semicolon.

    line x=p(0, 10) y=(25, 50) color=black;

## Comments

Single line:

    // this is comment

Part-line / multiline:

    /*
        multiline comment
    */

    let x = /* can be also in the middle of any expression */ 5;

Comments can be nested:

    /* /* */ */
    /* */ */
    /* /* */

Anything inside comments shouldn't break compilation.