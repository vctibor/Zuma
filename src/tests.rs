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
<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500">
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
fn test_scope() {
    let input = r#"
{
    rectangle start=[100,100] size=[300,200] color=red stroke-color=red;

    {
        line start=[100,100] end=[100,300] color=white width=15;
        line start=[400,100] end=[400,300] color=white width=15;
    }
}
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500">
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
<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500">
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
<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500">
    <rect height="50" style="stroke-width:4;stroke:rgb(255,255,255);fill:rgb(255,0,0);opacity:1" width="100" x="50" y="50"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}

// TODO: Test that all files in example folder do compile