#![allow(unused_imports)]

use crate::ZumaCompiler;

#[test]
fn test_function_call() {
    let input = r#"
rectangle start=[50,50] size=[100,100] color=blue opacity=0.3;
rectangle start=[10,10] size=[100,100] color=red opacity=0.3;
rectangle start=[70,40] size=[100,100] color=green opacity=0.3;
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <rect height="100" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(0,0,255);opacity:0.3" width="100" x="50" y="50"/>
    <rect height="100" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(255,0,0);opacity:0.3" width="100" x="10" y="10"/>
    <rect height="100" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(0,255,0);opacity:0.3" width="100" x="40" y="70"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}

#[test]
fn test_scopes_1() {
    let input = r#"
{
    rectangle start=[100,100] size=[300,200] color=red stroke-color=red;
    {
        line start=[100,100] end=[100,300] color=white width=15;
        line start=[400,100] end=[400,300] color=white width=15;
    };
};
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <rect height="300" style="stroke-width:1;stroke:rgb(255,0,0);fill:rgb(255,0,0);opacity:1" width="200" x="100" y="100"/>
    <line style="stroke-width:15;stroke:rgb(255,255,255)" x1="100" x2="300" y1="100" y2="100"/>
    <line style="stroke-width:15;stroke:rgb(255,255,255)" x1="100" x2="300" y1="400" y2="400"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}

#[test]
fn test_scopes_2() {
    let input = r#"
let size = [50, 50];
rectangle start=[10, 10] size=size color=white;
{
    rectangle start=[100, 10] size=size color=white;
    let size = [20, 80];
    rectangle start=[200, 10] size=size color=blue;
    {
        rectangle start=[300, 10] size=size color=red;
    };
};
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <rect height="50" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(255,255,255);opacity:1" width="50" x="10" y="10"/>
    <rect height="50" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(255,255,255);opacity:1" width="50" x="10" y="100"/>
    <rect height="20" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(0,0,255);opacity:1" width="80" x="10" y="200"/>
    <rect height="20" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(255,0,0);opacity:1" width="80" x="10" y="300"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}

#[test]
fn test_constants() {
    let input = r#"
let malky_green = #bbe088;
let purple = #994fd1;
let rectangle_size = [120, 120];
let my_width = 15;
rectangle start=[100,100] size=rectangle_size color=malky_green stroke-color=purple
    stroke-width=my_width;
rectangle start=[100,300] size=rectangle_size color=malky_green stroke-color=purple
    stroke-width=my_width;
rectangle start=[300,100] size=rectangle_size color=malky_green stroke-color=purple
    stroke-width=my_width;
rectangle start=[300,300] size=rectangle_size color=malky_green stroke-color=purple
    stroke-width=my_width;
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <rect height="120" style="stroke-width:15;stroke:rgb(153,79,209);fill:rgb(187,224,136);opacity:1" width="120" x="100" y="100"/>
    <rect height="120" style="stroke-width:15;stroke:rgb(153,79,209);fill:rgb(187,224,136);opacity:1" width="120" x="300" y="100"/>
    <rect height="120" style="stroke-width:15;stroke:rgb(153,79,209);fill:rgb(187,224,136);opacity:1" width="120" x="100" y="300"/>
    <rect height="120" style="stroke-width:15;stroke:rgb(153,79,209);fill:rgb(187,224,136);opacity:1" width="120" x="300" y="300"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}

#[test]
fn test_bools() {
    let input = r#"
let ab = true;
let ba = false;
let w = 4;
rectangle start=[50,50] size=[50,100] color=red stroke-width=w stroke-color=white;
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <rect height="50" style="stroke-width:4;stroke:rgb(255,255,255);fill:rgb(255,0,0);opacity:1" width="100" x="50" y="50"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}

#[test]
fn test_operations() {
    let input = r#"
let width = (3 + 5) * (10 * 2);
line start=[50, 200] end=[150,350] width=width color=red;
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <line style="stroke-width:160;stroke:rgb(255,0,0)" x1="200" x2="350" y1="50" y2="150"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}

#[test]
fn test_for_loop() {
    let input = r#"
let start_x = 0;
let start_y = 100;
let length = 180;
let distance = 20;
for index = 1, 1, 5 {
    let start_x = (start_x + (index * distance));
    let end_x = start_x + length;
    let end_y = start_y + length;
    line start=[start_x, start_y] end=[end_x, end_y] color=red width=3;
}
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <line style="stroke-width:3;stroke:rgb(255,0,0)" x1="100" x2="280" y1="20" y2="200"/>
    <line style="stroke-width:3;stroke:rgb(255,0,0)" x1="100" x2="280" y1="40" y2="220"/>
    <line style="stroke-width:3;stroke:rgb(255,0,0)" x1="100" x2="280" y1="60" y2="240"/>
    <line style="stroke-width:3;stroke:rgb(255,0,0)" x1="100" x2="280" y1="80" y2="260"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}




/// Because Zuma is - or at least we want it to be - expressions based language,
/// it should be possible to evaluate *any* piece of code and assign it to constant.
/// Then expression of type Graphics when invoked by itself (not used in any higher level construct)
/// should generate piece of SVG code.
/// It should be indifferent whether we invoked expression by its literal declaration,
/// or assigned it to constant and then invoked that constant.
#[ignore]   // not yet implemented
#[test]
fn test_expressions() {
    let input_1 = r#"
line start=[10,15] end=[25,50] color=green width=3;
    "#.trim();

    let input_2 = r#"
let amazing_line = line start=[10,15] end=[25,50] color=green width=3;

amazing_line;
    "#.trim();

    let compiler = ZumaCompiler::new();
    let output_1 = compiler.compile(input_1.to_owned()).unwrap();
    let output_2 = compiler.compile(input_2.to_owned()).unwrap();

    dbg!(&output_1);
    dbg!(&output_2);

    assert_eq!(output_1, output_2);
}

#[ignore]   // not yet implemented
#[test]
fn test_user_defined_procedure() {
    let input_1 = r#"
let arrow_start = [100, 100];

let pointing_to_x = 100;
let pointing_to_y = 200;

let head_height = 10;
let head_length = 20;

let arrow_color = green;
let arrow_width = 3;

let pointing_to = [
    pointing_to_x,
    pointing_to_y
];

let head_y = (pointing_to_y - head_length);

line start=arrow_start end=pointing_to color=arrow_color width=arrow_width;
line start=[(pointing_to_x - head_height), head_y] end=pointing_to color=arrow_color width=arrow_width;
line start=[(pointing_to_x + head_height), head_y] end=pointing_to color=arrow_color width=arrow_width;
    "#.trim();

    let input_2 = r#"
let arrow(
    arrow_start: Point,
    pointing_to_x: Number,
    pointing_to_y: Number,
    head_height: Number = 10,
    head_length: Number = 20,
    arrow_color: Color = black,
    arrow_width: Number = 3)
{
    let pointing_to = [
        pointing_to_x,
        pointing_to_y
    ];

    let head_y = (pointing_to_y - head_length);

    line start=arrow_start end=pointing_to color=arrow_color width=arrow_width;
    line start=[(pointing_to_x - head_height), head_y] end=pointing_to color=arrow_color width=arrow_width;
    line start=[(pointing_to_x + head_height), head_y] end=pointing_to color=arrow_color width=arrow_width;
};


let arrow_start = [100, 100];

let pointing_to_x = 100;
let pointing_to_y = 200;

let head_height = 10;
let head_length = 20;

let arrow_color = green;
let arrow_width = 3;

arrow arrow_start = arrow_start
        pointing_to_x = pointing_to_x
        pointing_to_y = pointing_to_y
        head_height = head_height
        head_length = head_length
        arrow_color = arrow_color
        arrow_width = arrow_width;
    "#.trim();

    let compiler = ZumaCompiler::new();
    let output_1 = compiler.compile(input_1.to_owned()).unwrap();
    let output_2 = compiler.compile(input_2.to_owned()).unwrap();

    dbg!(&output_1);
    dbg!(&output_2);

    assert_eq!(output_1, output_2);
}