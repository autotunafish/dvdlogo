extern crate drawille;
use drawille::Canvas;
use drawille::PixelColor::TrueColor;

pub fn border(cv: &Vec<(u8, u8, u8)>, mut c: Canvas, s: u32, a: u32) -> Canvas {
    let r = cv[0].0;
    let g = cv[0].1;
    let b = cv[0].2;

    c.line_colored(1, 1, 1, a + 1, TrueColor { r: r, g: g, b: b });
    c.line_colored(1, 1, s + 1, 1, TrueColor { r: r, g: g, b: b });
    c.line_colored(1, a + 1, s + 1, a + 1, TrueColor { r: r, g: g, b: b });
    c.line_colored(s + 1, 1, s + 1, a + 1, TrueColor { r: r, g: g, b: b });
    c
}

pub fn filllogoborder(x: i32, y: i32) -> Vec<(i32, i32)> {
    //DVD logo dimensions: 105, 43

    let mut d = Vec::new();

    for i in 0..106 {
        d.push((x + i, y));
        d.push((x + i, y + 44));
    }

    for i in 0..44 {
        d.push((x, y + i));
        d.push((x + 106, y + i));
    }

    d
}

//Draws the DVD logo as a filled object.
pub fn fulldvdpic(cv: &Vec<(u8, u8, u8)>, mut canvas: Canvas, o: [i32; 2]) -> Canvas {
    //Grabs the color values.
    let r = cv[0].0;
    let g = cv[0].1;
    let b = cv[0].2;

    //Grabs the coords
    let mut ox: u32 = o[0].try_into().unwrap();
    let oy: u32 = o[1].try_into().unwrap();

    //Plus one for a single line vertical buffer.
    ox += 1;

    //cargo fmt likes to make it like this...
    //2
    canvas.line_colored(
        ox + 12,
        oy + 1,
        ox + 45,
        oy + 1,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 1,
        ox + 91,
        oy + 1,
        TrueColor { r: r, g: g, b: b },
    );

    //3
    canvas.line_colored(
        ox + 12,
        oy + 2,
        ox + 46,
        oy + 2,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 64,
        oy + 2,
        ox + 96,
        oy + 2,
        TrueColor { r: r, g: g, b: b },
    );

    //4
    canvas.line_colored(
        ox + 11,
        oy + 3,
        ox + 46,
        oy + 3,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 63,
        oy + 3,
        ox + 98,
        oy + 3,
        TrueColor { r: r, g: g, b: b },
    );

    //5
    canvas.line_colored(
        ox + 11,
        oy + 4,
        ox + 47,
        oy + 4,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 62,
        oy + 4,
        ox + 100,
        oy + 4,
        TrueColor { r: r, g: g, b: b },
    );

    //6
    canvas.line_colored(
        ox + 11,
        oy + 5,
        ox + 47,
        oy + 5,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 61,
        oy + 5,
        ox + 101,
        oy + 5,
        TrueColor { r: r, g: g, b: b },
    );

    //7
    canvas.line_colored(
        ox + 27,
        oy + 6,
        ox + 48,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 60,
        oy + 6,
        ox + 72,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 89,
        oy + 6,
        ox + 102,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );

    //8
    canvas.line_colored(
        ox + 9,
        oy + 7,
        ox + 20,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 29,
        oy + 7,
        ox + 48,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 59,
        oy + 7,
        ox + 71,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 73,
        oy + 7,
        ox + 84,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 91,
        oy + 7,
        ox + 103,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );

    //8
    canvas.line_colored(
        ox + 9,
        oy + 8,
        ox + 20,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 30,
        oy + 8,
        ox + 40,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 42,
        oy + 8,
        ox + 49,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 58,
        oy + 8,
        ox + 70,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 73,
        oy + 8,
        ox + 84,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 92,
        oy + 8,
        ox + 103,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );

    //9
    canvas.line_colored(
        ox + 8,
        oy + 9,
        ox + 19,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 31,
        oy + 9,
        ox + 49,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 57,
        oy + 9,
        ox + 69,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 72,
        oy + 9,
        ox + 83,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 93,
        oy + 9,
        ox + 104,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );

    //10
    canvas.line_colored(
        ox + 8,
        oy + 10,
        ox + 19,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 32,
        oy + 10,
        ox + 41,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 43,
        oy + 10,
        ox + 50,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 56,
        oy + 10,
        ox + 68,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 72,
        oy + 10,
        ox + 83,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 94,
        oy + 10,
        ox + 104,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    //11
    canvas.line_colored(
        ox + 7,
        oy + 11,
        ox + 18,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 32,
        oy + 11,
        ox + 41,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 43,
        oy + 11,
        ox + 50,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 55,
        oy + 11,
        ox + 67,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 71,
        oy + 11,
        ox + 82,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 94,
        oy + 11,
        ox + 104,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    //12
    canvas.line_colored(
        ox + 7,
        oy + 12,
        ox + 18,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 32,
        oy + 12,
        ox + 41,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 44,
        oy + 12,
        ox + 51,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 54,
        oy + 12,
        ox + 66,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 71,
        oy + 12,
        ox + 82,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 94,
        oy + 12,
        ox + 104,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    //13
    canvas.line_colored(
        ox + 6,
        oy + 13,
        ox + 17,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 32,
        oy + 13,
        ox + 41,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 44,
        oy + 13,
        ox + 51,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 53,
        oy + 13,
        ox + 65,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 70,
        oy + 13,
        ox + 81,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 94,
        oy + 13,
        ox + 104,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    ///////////////////////
    //14
    canvas.line_colored(
        ox + 6,
        oy + 14,
        ox + 17,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 31,
        oy + 14,
        ox + 41,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 45,
        oy + 14,
        ox + 64,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 70,
        oy + 14,
        ox + 81,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 93,
        oy + 14,
        ox + 104,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );

    //15
    canvas.line_colored(
        ox + 5,
        oy + 15,
        ox + 16,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 30,
        oy + 15,
        ox + 41,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 45,
        oy + 15,
        ox + 63,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 69,
        oy + 15,
        ox + 80,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 92,
        oy + 15,
        ox + 104,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );

    //16
    canvas.line_colored(
        ox + 5,
        oy + 16,
        ox + 16,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 29,
        oy + 16,
        ox + 40,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 46,
        oy + 16,
        ox + 62,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 69,
        oy + 16,
        ox + 80,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 91,
        oy + 16,
        ox + 103,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );

    //17
    canvas.line_colored(
        ox + 4,
        oy + 17,
        ox + 15,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 27,
        oy + 17,
        ox + 40,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 46,
        oy + 17,
        ox + 61,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 68,
        oy + 17,
        ox + 79,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 89,
        oy + 17,
        ox + 103,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );

    //18
    canvas.line_colored(
        ox + 4,
        oy + 18,
        ox + 39,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 47,
        oy + 18,
        ox + 60,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 68,
        oy + 18,
        ox + 102,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );

    /////////////////////////
    ///////////////////////////
    /////////////////////
    //19
    canvas.line_colored(
        ox + 3,
        oy + 19,
        ox + 38,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 47,
        oy + 19,
        ox + 59,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 67,
        oy + 19,
        ox + 101,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );

    //20
    canvas.line_colored(
        ox + 3,
        oy + 20,
        ox + 37,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 48,
        oy + 20,
        ox + 58,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 67,
        oy + 20,
        ox + 100,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );

    //21
    canvas.line_colored(
        ox + 2,
        oy + 21,
        ox + 35,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 48,
        oy + 21,
        ox + 57,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 66,
        oy + 21,
        ox + 98,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );

    //22
    canvas.line_colored(
        ox + 2,
        oy + 22,
        ox + 33,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 49,
        oy + 22,
        ox + 56,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 66,
        oy + 22,
        ox + 96,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );

    //23
    canvas.line_colored(
        ox + 2,
        oy + 23,
        ox + 30,
        oy + 23,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 49,
        oy + 23,
        ox + 55,
        oy + 23,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 66,
        oy + 23,
        ox + 93,
        oy + 23,
        TrueColor { r: r, g: g, b: b },
    );

    //24
    canvas.line_colored(
        ox + 50,
        oy + 24,
        ox + 54,
        oy + 24,
        TrueColor { r: r, g: g, b: b },
    );

    //25
    canvas.line_colored(
        ox + 50,
        oy + 25,
        ox + 53,
        oy + 25,
        TrueColor { r: r, g: g, b: b },
    );

    //26
    canvas.line_colored(
        ox + 51,
        oy + 26,
        ox + 52,
        oy + 26,
        TrueColor { r: r, g: g, b: b },
    );

    //27
    canvas.line_colored(
        ox + 34,
        oy + 27,
        ox + 69,
        oy + 27,
        TrueColor { r: r, g: g, b: b },
    );

    //28
    canvas.line_colored(
        ox + 22,
        oy + 28,
        ox + 81,
        oy + 28,
        TrueColor { r: r, g: g, b: b },
    );

    //29
    canvas.line_colored(
        ox + 14,
        oy + 29,
        ox + 89,
        oy + 29,
        TrueColor { r: r, g: g, b: b },
    );

    //30
    canvas.line_colored(
        ox + 9,
        oy + 30,
        ox + 94,
        oy + 30,
        TrueColor { r: r, g: g, b: b },
    );

    //31
    canvas.line_colored(
        ox + 6,
        oy + 31,
        ox + 97,
        oy + 31,
        TrueColor { r: r, g: g, b: b },
    );

    //32
    canvas.line_colored(
        ox + 3,
        oy + 32,
        ox + 100,
        oy + 32,
        TrueColor { r: r, g: g, b: b },
    );

    //33
    canvas.line_colored(
        ox + 1,
        oy + 33,
        ox + 42,
        oy + 33,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 60,
        oy + 33,
        ox + 102,
        oy + 33,
        TrueColor { r: r, g: g, b: b },
    );

    //34
    canvas.line_colored(
        ox + 0,
        oy + 34,
        ox + 37,
        oy + 34,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 34,
        ox + 103,
        oy + 34,
        TrueColor { r: r, g: g, b: b },
    );

    //35
    canvas.line_colored(
        ox + 0,
        oy + 35,
        ox + 37,
        oy + 35,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 35,
        ox + 103,
        oy + 35,
        TrueColor { r: r, g: g, b: b },
    );

    //36
    canvas.line_colored(
        ox + 0,
        oy + 36,
        ox + 37,
        oy + 36,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 36,
        ox + 103,
        oy + 36,
        TrueColor { r: r, g: g, b: b },
    );

    //37
    canvas.line_colored(
        ox + 1,
        oy + 37,
        ox + 42,
        oy + 37,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 60,
        oy + 37,
        ox + 102,
        oy + 37,
        TrueColor { r: r, g: g, b: b },
    );

    //38
    canvas.line_colored(
        ox + 3,
        oy + 38,
        ox + 100,
        oy + 38,
        TrueColor { r: r, g: g, b: b },
    );

    //39
    canvas.line_colored(
        ox + 6,
        oy + 39,
        ox + 97,
        oy + 39,
        TrueColor { r: r, g: g, b: b },
    );

    //40
    canvas.line_colored(
        ox + 9,
        oy + 40,
        ox + 94,
        oy + 40,
        TrueColor { r: r, g: g, b: b },
    );

    //41
    canvas.line_colored(
        ox + 14,
        oy + 41,
        ox + 89,
        oy + 41,
        TrueColor { r: r, g: g, b: b },
    );

    //42
    canvas.line_colored(
        ox + 22,
        oy + 42,
        ox + 81,
        oy + 42,
        TrueColor { r: r, g: g, b: b },
    );

    //43
    canvas.line_colored(
        ox + 34,
        oy + 43,
        ox + 69,
        oy + 43,
        TrueColor { r: r, g: g, b: b },
    );

    return canvas;
}

//Draws the outline of the DVD logo.
#[allow(dead_code)]
pub fn dvdpic(cv: &Vec<(u8, u8, u8)>, mut canvas: Canvas, o: [i32; 2]) -> Canvas {
    //Grabs the color values.
    let r = cv[0].0;
    let g = cv[0].1;
    let b = cv[0].2;

    //Grabs the coords
    let mut ox: u32 = o[0].try_into().unwrap();
    let oy: u32 = o[1].try_into().unwrap();

    //Plus one for a single line vertical buffer.
    ox += 1;

    //cargo fmt likes to make it like this...
    //2
    canvas.line_colored(
        ox + 12,
        oy + 1,
        ox + 45,
        oy + 1,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 1,
        ox + 91,
        oy + 1,
        TrueColor { r: r, g: g, b: b },
    );

    //3
    canvas.line_colored(
        ox + 12,
        oy + 2,
        ox + 12,
        oy + 2,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 46,
        oy + 2,
        ox + 46,
        oy + 2,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 64,
        oy + 2,
        ox + 64,
        oy + 2,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 91,
        oy + 2,
        ox + 96,
        oy + 2,
        TrueColor { r: r, g: g, b: b },
    );

    //4
    canvas.line_colored(
        ox + 11,
        oy + 3,
        ox + 12,
        oy + 3,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 46,
        oy + 3,
        ox + 46,
        oy + 3,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 63,
        oy + 3,
        ox + 63,
        oy + 3,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 96,
        oy + 3,
        ox + 98,
        oy + 3,
        TrueColor { r: r, g: g, b: b },
    );

    //5
    canvas.line_colored(
        ox + 11,
        oy + 4,
        ox + 11,
        oy + 4,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 47,
        oy + 4,
        ox + 47,
        oy + 4,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 62,
        oy + 4,
        ox + 62,
        oy + 4,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 98,
        oy + 4,
        ox + 100,
        oy + 4,
        TrueColor { r: r, g: g, b: b },
    );

    //6
    canvas.line_colored(
        ox + 11,
        oy + 5,
        ox + 26,
        oy + 5,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 47,
        oy + 5,
        ox + 47,
        oy + 5,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 61,
        oy + 5,
        ox + 61,
        oy + 5,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 73,
        oy + 5,
        ox + 89,
        oy + 5,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 100,
        oy + 5,
        ox + 101,
        oy + 5,
        TrueColor { r: r, g: g, b: b },
    );

    //7
    canvas.line_colored(
        ox + 26,
        oy + 6,
        ox + 29,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 40,
        oy + 6,
        ox + 40,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 48,
        oy + 6,
        ox + 48,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 60,
        oy + 6,
        ox + 60,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 72,
        oy + 6,
        ox + 72,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 89,
        oy + 6,
        ox + 91,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 101,
        oy + 6,
        ox + 102,
        oy + 6,
        TrueColor { r: r, g: g, b: b },
    );

    //8
    canvas.line_colored(
        ox + 9,
        oy + 7,
        ox + 20,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 29,
        oy + 7,
        ox + 30,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 40,
        oy + 7,
        ox + 41,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 48,
        oy + 7,
        ox + 48,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 59,
        oy + 7,
        ox + 59,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 71,
        oy + 7,
        ox + 71,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 73,
        oy + 7,
        ox + 84,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 91,
        oy + 7,
        ox + 92,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 102,
        oy + 7,
        ox + 103,
        oy + 7,
        TrueColor { r: r, g: g, b: b },
    );

    //8
    canvas.line_colored(
        ox + 9,
        oy + 8,
        ox + 9,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 20,
        oy + 8,
        ox + 20,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 30,
        oy + 8,
        ox + 31,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 40,
        oy + 8,
        ox + 40,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 42,
        oy + 8,
        ox + 42,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 49,
        oy + 8,
        ox + 49,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 58,
        oy + 8,
        ox + 58,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 70,
        oy + 8,
        ox + 70,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 73,
        oy + 8,
        ox + 73,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 84,
        oy + 8,
        ox + 84,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 92,
        oy + 8,
        ox + 93,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 103,
        oy + 8,
        ox + 103,
        oy + 8,
        TrueColor { r: r, g: g, b: b },
    );

    //9
    canvas.line_colored(
        ox + 8,
        oy + 9,
        ox + 9,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 19,
        oy + 9,
        ox + 19,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 31,
        oy + 9,
        ox + 32,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 41,
        oy + 9,
        ox + 42,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 49,
        oy + 9,
        ox + 49,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 57,
        oy + 9,
        ox + 57,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 69,
        oy + 9,
        ox + 69,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 72,
        oy + 9,
        ox + 73,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 83,
        oy + 9,
        ox + 83,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 93,
        oy + 9,
        ox + 94,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 103,
        oy + 9,
        ox + 104,
        oy + 9,
        TrueColor { r: r, g: g, b: b },
    );

    //10
    canvas.line_colored(
        ox + 8,
        oy + 10,
        ox + 8,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 19,
        oy + 10,
        ox + 19,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 32,
        oy + 10,
        ox + 32,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 41,
        oy + 10,
        ox + 41,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 43,
        oy + 10,
        ox + 43,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 50,
        oy + 10,
        ox + 50,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 56,
        oy + 10,
        ox + 56,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 68,
        oy + 10,
        ox + 68,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 72,
        oy + 10,
        ox + 72,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 83,
        oy + 10,
        ox + 83,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 94,
        oy + 10,
        ox + 94,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 104,
        oy + 10,
        ox + 104,
        oy + 10,
        TrueColor { r: r, g: g, b: b },
    );

    //11
    canvas.line_colored(
        ox + 7,
        oy + 11,
        ox + 8,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 18,
        oy + 11,
        ox + 18,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 32,
        oy + 11,
        ox + 32,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 41,
        oy + 11,
        ox + 41,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 43,
        oy + 11,
        ox + 43,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 50,
        oy + 11,
        ox + 50,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 55,
        oy + 11,
        ox + 55,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 67,
        oy + 11,
        ox + 67,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 71,
        oy + 11,
        ox + 72,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 82,
        oy + 11,
        ox + 82,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 94,
        oy + 11,
        ox + 94,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 104,
        oy + 11,
        ox + 104,
        oy + 11,
        TrueColor { r: r, g: g, b: b },
    );

    //12
    canvas.line_colored(
        ox + 7,
        oy + 12,
        ox + 7,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 18,
        oy + 12,
        ox + 18,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 32,
        oy + 12,
        ox + 32,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 41,
        oy + 12,
        ox + 41,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 44,
        oy + 12,
        ox + 44,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 51,
        oy + 12,
        ox + 51,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 54,
        oy + 12,
        ox + 54,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 66,
        oy + 12,
        ox + 66,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 71,
        oy + 12,
        ox + 71,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 82,
        oy + 12,
        ox + 82,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 94,
        oy + 12,
        ox + 94,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 104,
        oy + 12,
        ox + 104,
        oy + 12,
        TrueColor { r: r, g: g, b: b },
    );

    //13
    canvas.line_colored(
        ox + 6,
        oy + 13,
        ox + 7,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 17,
        oy + 13,
        ox + 17,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 32,
        oy + 13,
        ox + 32,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 41,
        oy + 13,
        ox + 41,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 44,
        oy + 13,
        ox + 44,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 51,
        oy + 13,
        ox + 51,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 53,
        oy + 13,
        ox + 53,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 13,
        ox + 65,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 70,
        oy + 13,
        ox + 71,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 81,
        oy + 13,
        ox + 81,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 94,
        oy + 13,
        ox + 94,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 104,
        oy + 13,
        ox + 104,
        oy + 13,
        TrueColor { r: r, g: g, b: b },
    );

    //14
    canvas.line_colored(
        ox + 6,
        oy + 14,
        ox + 6,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 17,
        oy + 14,
        ox + 17,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 31,
        oy + 14,
        ox + 32,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 41,
        oy + 14,
        ox + 41,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 45,
        oy + 14,
        ox + 45,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 52,
        oy + 14,
        ox + 52,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 64,
        oy + 14,
        ox + 64,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 70,
        oy + 14,
        ox + 70,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 81,
        oy + 14,
        ox + 81,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 93,
        oy + 14,
        ox + 94,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 104,
        oy + 14,
        ox + 104,
        oy + 14,
        TrueColor { r: r, g: g, b: b },
    );

    //15
    canvas.line_colored(
        ox + 5,
        oy + 15,
        ox + 6,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 16,
        oy + 15,
        ox + 16,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 30,
        oy + 15,
        ox + 31,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 40,
        oy + 15,
        ox + 41,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 45,
        oy + 15,
        ox + 45,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 63,
        oy + 15,
        ox + 63,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 69,
        oy + 15,
        ox + 70,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 80,
        oy + 15,
        ox + 80,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 92,
        oy + 15,
        ox + 93,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 103,
        oy + 15,
        ox + 104,
        oy + 15,
        TrueColor { r: r, g: g, b: b },
    );

    //16
    canvas.line_colored(
        ox + 5,
        oy + 16,
        ox + 5,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 16,
        oy + 16,
        ox + 16,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 29,
        oy + 16,
        ox + 30,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 40,
        oy + 16,
        ox + 40,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 46,
        oy + 16,
        ox + 46,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 62,
        oy + 16,
        ox + 62,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 69,
        oy + 16,
        ox + 69,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 80,
        oy + 16,
        ox + 80,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 91,
        oy + 16,
        ox + 92,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 103,
        oy + 16,
        ox + 103,
        oy + 16,
        TrueColor { r: r, g: g, b: b },
    );

    //17
    canvas.line_colored(
        ox + 4,
        oy + 17,
        ox + 5,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 15,
        oy + 17,
        ox + 15,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 27,
        oy + 17,
        ox + 29,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 39,
        oy + 17,
        ox + 40,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 46,
        oy + 17,
        ox + 46,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 61,
        oy + 17,
        ox + 61,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 68,
        oy + 17,
        ox + 69,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 79,
        oy + 17,
        ox + 79,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 89,
        oy + 17,
        ox + 91,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 102,
        oy + 17,
        ox + 103,
        oy + 17,
        TrueColor { r: r, g: g, b: b },
    );

    //18
    canvas.line_colored(
        ox + 4,
        oy + 18,
        ox + 4,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 15,
        oy + 18,
        ox + 27,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 38,
        oy + 18,
        ox + 39,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 47,
        oy + 18,
        ox + 47,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 60,
        oy + 18,
        ox + 60,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 68,
        oy + 18,
        ox + 68,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 79,
        oy + 18,
        ox + 89,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 101,
        oy + 18,
        ox + 102,
        oy + 18,
        TrueColor { r: r, g: g, b: b },
    );

    //19
    canvas.line_colored(
        ox + 3,
        oy + 19,
        ox + 4,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 37,
        oy + 19,
        ox + 38,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 47,
        oy + 19,
        ox + 47,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 59,
        oy + 19,
        ox + 59,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 67,
        oy + 19,
        ox + 68,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 100,
        oy + 19,
        ox + 101,
        oy + 19,
        TrueColor { r: r, g: g, b: b },
    );

    //20
    canvas.line_colored(
        ox + 3,
        oy + 20,
        ox + 3,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 35,
        oy + 20,
        ox + 37,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 48,
        oy + 20,
        ox + 48,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 58,
        oy + 20,
        ox + 58,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 67,
        oy + 20,
        ox + 67,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 98,
        oy + 20,
        ox + 100,
        oy + 20,
        TrueColor { r: r, g: g, b: b },
    );

    //21
    canvas.line_colored(
        ox + 2,
        oy + 21,
        ox + 3,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 33,
        oy + 21,
        ox + 35,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 48,
        oy + 21,
        ox + 48,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );

    canvas.line_colored(
        ox + 57,
        oy + 21,
        ox + 57,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 66,
        oy + 21,
        ox + 67,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 96,
        oy + 21,
        ox + 98,
        oy + 21,
        TrueColor { r: r, g: g, b: b },
    );

    //22
    canvas.line_colored(
        ox + 2,
        oy + 22,
        ox + 2,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 30,
        oy + 22,
        ox + 33,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 49,
        oy + 22,
        ox + 49,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 56,
        oy + 22,
        ox + 56,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 66,
        oy + 22,
        ox + 66,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 93,
        oy + 22,
        ox + 96,
        oy + 22,
        TrueColor { r: r, g: g, b: b },
    );

    //23
    canvas.line_colored(
        ox + 2,
        oy + 23,
        ox + 30,
        oy + 23,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 49,
        oy + 23,
        ox + 49,
        oy + 23,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 55,
        oy + 23,
        ox + 55,
        oy + 23,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 66,
        oy + 23,
        ox + 93,
        oy + 23,
        TrueColor { r: r, g: g, b: b },
    );

    //24
    canvas.line_colored(
        ox + 50,
        oy + 24,
        ox + 50,
        oy + 24,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 54,
        oy + 24,
        ox + 54,
        oy + 24,
        TrueColor { r: r, g: g, b: b },
    );

    //25
    canvas.line_colored(
        ox + 50,
        oy + 25,
        ox + 50,
        oy + 25,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 53,
        oy + 25,
        ox + 53,
        oy + 25,
        TrueColor { r: r, g: g, b: b },
    );

    //26
    canvas.line_colored(
        ox + 51,
        oy + 26,
        ox + 52,
        oy + 26,
        TrueColor { r: r, g: g, b: b },
    );

    //27
    canvas.line_colored(
        ox + 34,
        oy + 27,
        ox + 69,
        oy + 27,
        TrueColor { r: r, g: g, b: b },
    );

    //28
    canvas.line_colored(
        ox + 22,
        oy + 28,
        ox + 33,
        oy + 28,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 70,
        oy + 28,
        ox + 81,
        oy + 28,
        TrueColor { r: r, g: g, b: b },
    );

    //29
    canvas.line_colored(
        ox + 14,
        oy + 29,
        ox + 21,
        oy + 29,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 82,
        oy + 29,
        ox + 89,
        oy + 29,
        TrueColor { r: r, g: g, b: b },
    );

    //30
    canvas.line_colored(
        ox + 9,
        oy + 30,
        ox + 13,
        oy + 30,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 90,
        oy + 30,
        ox + 94,
        oy + 30,
        TrueColor { r: r, g: g, b: b },
    );

    //31
    canvas.line_colored(
        ox + 6,
        oy + 31,
        ox + 8,
        oy + 31,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 95,
        oy + 31,
        ox + 97,
        oy + 31,
        TrueColor { r: r, g: g, b: b },
    );

    //32
    canvas.line_colored(
        ox + 3,
        oy + 32,
        ox + 5,
        oy + 32,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 43,
        oy + 32,
        ox + 59,
        oy + 32,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 98,
        oy + 32,
        ox + 100,
        oy + 32,
        TrueColor { r: r, g: g, b: b },
    );

    //33
    canvas.line_colored(
        ox + 1,
        oy + 33,
        ox + 2,
        oy + 33,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 38,
        oy + 33,
        ox + 42,
        oy + 33,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 60,
        oy + 33,
        ox + 64,
        oy + 33,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 101,
        oy + 33,
        ox + 102,
        oy + 33,
        TrueColor { r: r, g: g, b: b },
    );

    //34
    canvas.line_colored(
        ox + 0,
        oy + 34,
        ox + 0,
        oy + 34,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 36,
        oy + 34,
        ox + 37,
        oy + 34,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 34,
        ox + 66,
        oy + 34,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 103,
        oy + 34,
        ox + 103,
        oy + 34,
        TrueColor { r: r, g: g, b: b },
    );

    //35
    canvas.line_colored(
        ox + 0,
        oy + 35,
        ox + 0,
        oy + 35,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 36,
        oy + 35,
        ox + 37,
        oy + 35,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 35,
        ox + 66,
        oy + 35,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 103,
        oy + 35,
        ox + 103,
        oy + 35,
        TrueColor { r: r, g: g, b: b },
    );

    //36
    canvas.line_colored(
        ox + 0,
        oy + 36,
        ox + 0,
        oy + 36,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 36,
        oy + 36,
        ox + 37,
        oy + 36,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 65,
        oy + 36,
        ox + 66,
        oy + 36,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 103,
        oy + 36,
        ox + 103,
        oy + 36,
        TrueColor { r: r, g: g, b: b },
    );

    //37
    canvas.line_colored(
        ox + 1,
        oy + 37,
        ox + 2,
        oy + 37,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 38,
        oy + 37,
        ox + 42,
        oy + 37,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 60,
        oy + 37,
        ox + 64,
        oy + 37,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 101,
        oy + 37,
        ox + 102,
        oy + 37,
        TrueColor { r: r, g: g, b: b },
    );

    //38
    canvas.line_colored(
        ox + 3,
        oy + 38,
        ox + 5,
        oy + 38,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 43,
        oy + 38,
        ox + 59,
        oy + 38,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 98,
        oy + 38,
        ox + 100,
        oy + 38,
        TrueColor { r: r, g: g, b: b },
    );

    //39
    canvas.line_colored(
        ox + 6,
        oy + 39,
        ox + 8,
        oy + 39,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 95,
        oy + 39,
        ox + 97,
        oy + 39,
        TrueColor { r: r, g: g, b: b },
    );

    //40
    canvas.line_colored(
        ox + 9,
        oy + 40,
        ox + 13,
        oy + 40,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 90,
        oy + 40,
        ox + 94,
        oy + 40,
        TrueColor { r: r, g: g, b: b },
    );

    //41
    canvas.line_colored(
        ox + 14,
        oy + 41,
        ox + 21,
        oy + 41,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 82,
        oy + 41,
        ox + 89,
        oy + 41,
        TrueColor { r: r, g: g, b: b },
    );

    //42
    canvas.line_colored(
        ox + 22,
        oy + 42,
        ox + 33,
        oy + 42,
        TrueColor { r: r, g: g, b: b },
    );
    canvas.line_colored(
        ox + 70,
        oy + 42,
        ox + 81,
        oy + 42,
        TrueColor { r: r, g: g, b: b },
    );

    //43
    canvas.line_colored(
        ox + 34,
        oy + 43,
        ox + 69,
        oy + 43,
        TrueColor { r: r, g: g, b: b },
    );

    return canvas;
}
