ZUMA - Vector Graphics Language
===============================

## WARNING: Language spec is currently unstable and WILL break.

[Playground](https://zuma-playground.netlify.app/)

ZUMA is Vector Graphics Language, i.e. domain specific language for constructing vector graphics. One could think about it like a graphics editor in the vein of *Inkscape*, just using slightly different kind of User Interface - namely plain text containing source code in some machine readable form, rather than Graphical User Interface more usual for these kinds of programs. Alternatively, you can think about ZUMA as being to vector graphics like *tex* is to text documents or *OpenSCAD* is to CAD.

Probably closest competitor to ZUMA is *SVG*, another language / text format intended for description of vector graphics; in fact, SVG is direct compilation target of ZUMA. *SVG* has problem of being *XML*, therefore verbose and more on the machine end of human-readable / machine-readable spectrum, and no fun to write at all. Another possible competition for ZUMA is *DOT* and similar languages for describing diagrams and graphs. While there certainly is ambition to use ZUMA to draw diagrams and graphs and anything else, this is intended to be achieved through functions, procedures and modules created by users from primitives  provided by language itself.

The main goal for ZUMA design is to be simple and easy, these two characteristics being different, often orthogonal. Either way, geometric shapes easy to express in natural language should be  simple/easy to express in ZUMA as well. For example, if I would say in natural language something like:

"Red rectangle one hundred by four hundred."

I can express the same idea in ZUMA by writing:

    rectangle start=[0,0] size=[100,400] color=red;
    
Or if I wanted to draw "blue line from point 10, 50 to 50, 100 wide 4 points" I could express it in ZUMA like:

    let A = [10, 50];
    let B = [50, 100];
    let my_special_blue = #6db3ed;
    line start=A end=B color=my_special_blue width=4;

While being simple, minimalistic language, ZUMA should at the same time provide basic functionality for DRY programming, namely conditionals, loops and user defined procedures.

It should be noted that in its current state ZUMA is more of research project for me to learn about developing machine language, and less readily usable tool; but there is intention to develop it into one. Meanwhile, it is expected that syntax will change dramatically and it shouldn't, if it even can, be used for anything serious.

## Cheatsheet

### Data types
    
ZUMA is statically, strongly typed. There are five data types:

    Bool Number Point Color Text

### Constant declaration

Every data type can be initialized by literal and assigned to constant:

    let x = true; let x = false;    // Bool

    let n = 157.44;                 // Number

    let p = [55, 143];              // Point

    let c = red; let c = #f5407;    // Color

    let t = "你好，世界！";           // Text

### Operations

Operations are used to calculate values. Inputs for operations can be literals, constants and other operations. Operations are *always* enclosed by parenthesis.

ZUMA supports basic arithmetic and logical operations.

    let five = (1 + 4);                     // Number

    let not_true = (3 > five);              // Bool

    let this_true = (five == (2 * 2.5));    // Bool

### Conditional

Conditional execution is achieved using `if ... else` construct, `else` being optional. Argument to `if` has to be Bool or operation that evaluates to Bool.

    if (4 > 5) {
        text text = "This isn't rendered.";
    } else {
        text text = "This block is optional, but will be rendered.";
    };

### Loop

`for` loop in ZUMA has following form:

    for index = 1, 2, 10 {
        
    };

`index` is constant name to which will be assigned value of loop index, available as "constant" inside loop body. The three numbers following equal sign are, in order: `starting_value`, value assigned to `index` before first pass through loop; `step`, amount by which is `index` increased after every pass through loop; and `final_value`.

If `step` is positive number, looping is stoped once it is equal or greater than `final_value`.

if `step` is negative number, looping is stoped once it is equal or less than `final_value`.

### User defined procedures

Procedures are used to generate graphics. Invocation of procedure (either predefined or user defined) leads to adding all graphical elements to global "stack" of graphical elements to be rendered in resulting SVG / PNG file. Therefore procedures are cornerstone of ZUMA language, as it's main and only purpose is to render graphics.

Procedures can be defined by keyword `proc` followed by name and list of arguments. Argument can be either required, in which case its name is followed by colon and datatype specification, and when not provided during invocation it leads to error; or it can be optional, in which case its name is followed by equal sign and default value (either literal, constant or operation) and it's type is infered from default value.

    proc box
        start_x: Number
        start_y: Number
        upper: Text
        lower = ""
        size = [70, 200]
        fg_color = #444444
        bg_color = white
    {
        rectangle start=[start_x, start_y] size=size color=bg_color stroke-color=fg_color stroke-width=4;

        text text=upper start=[(start_x + 30), (start_y + 70)] color=fg_color;
        text text=lower start=[(start_x + 50), (start_y + 60)] color=fg_color;
    }

### Predefined procedures

There are few predefined procedures for generating basic graphical elements:

    line start: Point end: Point color=black width=1
    
    rectangle start: Point size: Point color=black troke-color=black stroke-width=1 opacity=1 round-corners=0

    text start: Point text: Text color=black

    ellipse center: Point radius: Point color=black stroke-color=black stroke-width=1 opacity=1
