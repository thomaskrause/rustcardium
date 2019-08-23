#![no_std]
#![no_main]

use core::fmt::Write;
use rustcardium;
use rustcardium::main;
use rustcardium::UART;
use rustcardium::bhi160::{SensorType, Sensor, SensorConfig};

const BLACK: rustcardium::Color = rustcardium::Color { r: 0, g: 0, b: 0 };

const WHITE: rustcardium::Color = rustcardium::Color {
    r: 255,
    g: 255,
    b: 255,
};

const RED: rustcardium::Color = rustcardium::Color { r: 255, g: 0, b: 0 };
const GREEN: rustcardium::Color = rustcardium::Color { r: 0, g: 255, b: 0 };
const BLUE: rustcardium::Color = rustcardium::Color { r: 0, g: 0, b: 255 };

main!(main);
fn main() {
    loop {
        match execute_with_error() {
            Ok(_) => {}
            Err(e) => writeln!(UART, "Error: {:?}", e).unwrap(),
        }
    }
}

fn execute_with_error() -> Result<(), rustcardium::Error> {

    let accel = Sensor::enable(SensorType::Accelerometer, SensorConfig {
        sample_buffer_len: 100,
        sample_rate: 100,
        dynamic_range: 2,
    });

    let display = rustcardium::display::Display::open()?;
    
    let mut i: u16= 0;

    let mut min_x = i16::max_value() as f32;
    let mut max_x = i16::min_value() as f32;

    let mut min_y = i16::max_value() as f32;
    let mut max_y = i16::min_value() as f32;

    let mut min_z = i16::max_value() as f32;
    let mut max_z = i16::min_value() as f32;

    display.clear(Some(WHITE))?;
    display.print("x", RED, WHITE, 0, 0)?;
    display.print("y", GREEN, WHITE, 20, 0)?;
    display.print("z", BLUE, WHITE, 40, 0)?;

    let mut last_x : Option<f32> = None;  
    let mut last_y : Option<f32> = None;
    let mut last_z : Option<f32> = None;
    
    loop {

        // clear old area (but not whole screen)
        display.line(i, 20, i, 80, WHITE, false, 1)?;
        if i > 0 {
            display.line(i-1, 20, i-1, 80, WHITE, false, 1)?;
        }

        let values = accel.read();

        
        for v in values {

            let x = v.x as f32;
            let y = v.y as f32;
            let z = v.z as f32;

            min_x = if x < min_x {x} else {min_x};
            max_x = if x > max_x {x} else {max_x};

            min_y = if y < min_y {y} else {min_y};
            max_y = if y > max_y {y} else {max_y};

            min_z = if z < min_z {z} else {min_z};
            max_z = if z > max_z {z} else {max_z};

            // start at 0
            let x = x - min_x;
            let y = y - min_y;
            let z = z - min_z;
            
            // normalize to 0:1
            let x : f32 = if max_x > min_x {
                x / (max_x - min_x)
            } else {
                x
            };
            let y : f32 = if max_y > min_y {
                y / (max_y - min_y)
            } else {
                y
            };
            let z : f32 = if max_z > min_z {
                z / (max_z - min_z)
            } else {
                z
            };
            // scale up to 20-40
            let x = (x * 20.0) + 20.0;
            // scale up to 40-60
            let y = (y * 20.0) + 40.0;
            // scale up to 60-80
            let z = (z * 20.0) + 60.0;

            if let Some(last_x) = last_x {                
                // draw line from line from last to this one
                display.line(i-1, last_x as u16, i, x as u16, RED, false, 1)?;
            }
            if let Some(last_y) = last_y {                
                // draw line from line from last to this one
                display.line(i-1, last_y as u16, i, y as u16, GREEN, false, 1)?;            }
            if let Some(last_z) = last_z {                
                // draw line from line from last to this one
                display.line(i-1, last_z as u16, i, z as u16, BLUE, false, 1)?;
            }
            
            last_x = Some(x);
            last_y = Some(y);
            last_z = Some(z);

            i += 1;

            if i >= 160 {
                i = 1;
            }
        }       

    

        display.update()?;    
    }
}
