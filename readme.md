ZUMA - Vector Graphics Language
===============================

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

## Datatypes

ZUMA is strongly typed.

Following datatypes can be created using literlas

**Boolean** - has one of values

    true false

**Number** - single precision floating point, i.e. f32

    1.5464

**Point** - is declared using two numbers inside square brackets

    [4.45 6.06]

**Color** - can be declared using sharp followed by hexadecimal value:

    #ff001a

additionally few basic colors can be declared by their name:

    black white red green blue yellow

## Expressions

Expressions are delimited using semicolon.

    line x=p(0, 10) y=(25, 50) color=black;