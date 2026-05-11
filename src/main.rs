use embedded_graphics::pixelcolor::{BinaryColor, PixelColor};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder};
use embedded_graphics::Pixel;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), core::convert::Infallible> {
    println!("Hello, world!");

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(1000, 600));
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("life", &output_settings);
    window.update(&mut display);
    let mut paused = false;
    let mut destroy = false;
    'running: loop {
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, mouse_btn } => {
                    if !destroy {
                        println!("Construction click at {:?} with {:?}", point, mouse_btn);
                        spawn_cell(point).draw(&mut display);
                    } else {
                        println!("Destruction click at {:?} with {:?}", point, mouse_btn);
                        destroy_cell(point).draw(&mut display);
                    }
                }
                SimulatorEvent::KeyDown {
                    keycode: Keycode::SPACE,
                    ..
                } => {
                    paused = !paused;
                }
                SimulatorEvent::KeyDown {
                    keycode: Keycode::X,
                    ..
                } => {
                    destroy = !destroy;
                }
                _ => {}
            }
        }

        // put whatever in between here
        if !paused {
            for x in 1..display.size().width - 1 {
                for y in 1..display.size().height - 1 {
                    apply_rules(
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        &mut display,
                    )
                    .draw(&mut display);
                }
            }
        }
        // window update must be the last thing
        window.update(&mut display);
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn spawn_cell(point: Point) -> Pixel<BinaryColor> {
    Pixel(point, BinaryColor::On)
}
fn destroy_cell(point: Point) -> Pixel<BinaryColor> {
    Pixel(point, BinaryColor::Off)
}

fn apply_rules(point: Point, display: &mut SimulatorDisplay<BinaryColor>) -> Pixel<BinaryColor> {
    let mut neighbor_count = 0;
    for x in 0..3 {
        for y in 0..3 {
            let analyze = Point::new(point.x - 1 + x, point.y - 1 + y);
            if display.get_pixel(analyze) == BinaryColor::On {
                neighbor_count += 1;
            }
        }
    }
    match neighbor_count {
        0..=1 => Pixel(point, BinaryColor::Off),
        3..=4 => Pixel(point, BinaryColor::On),
        5.. => Pixel(point, BinaryColor::Off),
        _ => Pixel(point, BinaryColor::Off),
    }
}
