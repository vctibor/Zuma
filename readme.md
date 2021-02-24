ZUMA - Vector Graphics Language
===============================

## WARNING: Language spec is currently unstable and WILL break.

ZUMA is Vector Graphics Language, i.e. domain specific language intended for creation of vector graphics. One could think about it like a graphics editor in the vein of *Inkscape*, just using slightly different kind of User Interface - namely plain text containing source code in some machine readable form, rather than Graphical User Interface more usual for these kinds of programs. Alternatively, you can think about ZUMA as being to vestor graphics like *tex* is to text documents or *OpenSCAD* is to CAD.

Probably closest competitor to ZUMA is *SVG*, another language / text format intended for description of vector graphics; in fact, SVG is direct compilation target of ZUMA. *SVG* has problem of being *XML*, therefore verbose and more on the machine end of human-readable / machine-readable spectrum, and no fun to write at all. Another possible competition for ZUMA is *DOT* and similar languages for describing diagrams and graphs. While there certainly is ambition to use ZUMA to draw diagrams and graphs and anything else, this is intended to be achieved through functions, procedures and modules created by users from primitive types provided by language itself.

The main goal for ZUMA design is to be simple and easy, these two characteristics being different, often orthogonal. Either way, geometric shapes easy to express in natural language should be  simple/easy to express in ZUMA as well. For example, if I would say in natural language something like:

"Red rectangle one hundred by four hundred."

I can express the same idea in ZUMA by writing:

    rectangle start=[0,0] size=[100,400] color=red;

It should be noted that in its current state ZUMA is more of research project for me to learn about developing machine language, and less readily usable tool; but there is intention to develop it into one. Meanwhile, it is expected that syntax will change dramatically and it shouldn't, if it even can, be used for anything serious.