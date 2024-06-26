extern crate drawille;
use std::vec;
use drawille::Canvas;

use std::thread::sleep;
use std::time::{Duration, Instant};

mod fncs;
use fncs::{border, filllogoborder, fulldvdpic};

fn main() {
    let frameconstant = Duration::new(0, 12960000);
    //870000
    //1720000 is really fast, 300 fps
    //3240000 is half of that
    //6480000
    //12960000 70 fps
    //14949900 ~60 fps
    //29989999 ~30 FPS
    let sleepconstant = Duration::new(0, 39600);
    //696960
    //Sets initial values for color. Increments 1 at a time so goes slow
    let mut rcounter = 76;
    let mut gcounter = 152;
    let mut bcounter = 222;

    //Overall dimensions of the canvas.
    let xdim: i32 = 400;
    let ydim: i32 = 300;

    //Start positin of the Logo.
    let mut origin = [75, 50];

    //Border points of the logo.
    let mut logoborder = vec![(0, 0)];

    //Determines the up/dpwn or left/right
    let mut horzmoz: u32 = 0;
    let mut vertmov: u32 = 0;

    //Tells if a collision occurs to change the above values.
    let mut horzswitch = 0;
    let mut vertswitch = 0;

    //One pixel is the range of collision checks.
    let rng = 1;

    //These are used for cycling colors of 0-255, might remove.
    let mut tick = 0;
    let mut rcont = 0;
    let mut gcont = 0;
    let mut bcont = 0;

    //Generates canvas with overall display dimensions.
    //canvas is a collection arrays that hold chars...
    let mut canvas = Canvas::new(xdim.try_into().unwrap(), ydim.try_into().unwrap());

    //Inf loop for the display. ctrl-c to stop
    loop {
        //Clears old canvas point data.
        canvas.clear();

        //Todo make an improved frame timer.
        let frametimer = Instant::now();

        //Clear the Terminal screen and set cursor position.
        //Sets Lower Left. It doesn't look bad.
        //print!("{}[2J", 27 as char);
        //Sets Upper Right.
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        //Vector of the color values.
        let color = vec![(rcounter, gcounter, bcounter)];
        //println!("{:?}", &color);

        //Clears the previous logo border points.
        logoborder.clear();

        //Draws the Overall border. Not actually necessary but pretty.
        canvas = border(
            &color,
            canvas,
            xdim.try_into().unwrap(),
            ydim.try_into().unwrap(),
        );

        //Get the border of the logo for collision detection
        logoborder = filllogoborder(origin[0], origin[1]);

        //Check for an overall collision.
        for i in &logoborder {
            if i.0 == 1 {
                horzmoz = horzswitch;
            }
            if i.0 == xdim {
                horzmoz = horzswitch;
            }
            if i.1 == 1 {
                vertmov = vertswitch;
            }
            if i.1 == ydim {
                vertmov = vertswitch;
            }
        }

        //Channge dir relative to x or y.
        if horzswitch == horzmoz {
            match horzmoz {
                0 => {
                    horzswitch = 1;
                }
                1 => {
                    horzswitch = 0;
                }
                _ => (),
            }
        }

        if vertswitch == vertmov {
            match vertmov {
                0 => {
                    vertswitch = 1;
                }
                1 => {
                    vertswitch = 0;
                }
                _ => (),
            }
        }
        match horzmoz {
            0 => origin[0] -= rng,
            1 => origin[0] += rng,

            _ => (),
        }
        match vertmov {
            0 => origin[1] -= rng,
            1 => origin[1] += rng,
            _ => (),
        }

        //Prints the logo inside the larger canvas
        canvas = fulldvdpic(&color, canvas, origin);

        //Prints the logo border, for testing, not very pretty.
        //for i in &logoborder {canvas.set(i.0.try_into().unwrap(), i.1.try_into().unwrap());}

        println!("{}", canvas.clone().frame());

        //To try to keep a consistent framerate, sleep until the frameconstant has elapsed.
        'sleep: loop {
            let framediff = frametimer.elapsed();
            //println!("framediff {:?} nano", &framediff);

            if framediff >= frameconstant {
                //println!("frame {} at {:?} millis", &framecounter, &gametimer.elapsed().as_millis());

                break 'sleep;
            }

            if framediff < frameconstant {
                sleep(Duration::from(sleepconstant));
                //println!("frame {} at {:?} millis", &framecounter, &gametimer.elapsed().as_millis());
            }
        }

        //Color cycler, may remove in favor of set color pallette.
        match tick {
            0 => {
                bcounter += 1;
                if bcounter >= 255 {
                    bcont += 1;
                    bcounter = bcont;
                }
            }
            1 => {
                gcounter += 1;
                if gcounter >= 255 {
                    gcont += 1;
                    gcounter = gcont;
                }
            }
            2 => {
                rcounter += 1;
                if rcounter >= 255 {
                    rcont += 1;
                    rcounter = rcont;
                }
            }

            _ => (),
        }

        tick += 1;
        if tick >= 3 {
            tick = 0;
        }

        rcont += 1;
        if rcont >= 255 {
            rcont = 0;
            gcont += 1;
        }

        if gcont >= 255 {
            gcont = 0;
            bcont += 1;
        }

        if bcont >= 255 {
            bcont = 0;
        }
        //break;
    }
}

