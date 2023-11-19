mod modules {
    pub mod vector2;
    pub mod vector2i;
}

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use pixels::{Pixels, SurfaceTexture};

const STANDART_R: u8 = 0x5c;
const STANDART_G: u8 = 0x5c;
const STANDART_B: u8 = 0x5c;
const STANDART_A: u8 = 0xff;

#[derive(Clone, Copy, Debug, Default)]
struct Pixel {
    x: i64,
    y: i64,
    empty: bool,
}

impl Pixel {
    pub fn new(x: i64, y: i64, empty: bool) -> Pixel {
        return Pixel {
            x: x,
            y: y,
            empty: empty,
        }
    }

    pub fn set_empty(&mut self) {
        self.empty = true;
    }

    pub fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

struct CelluralMatrix {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl CelluralMatrix {
    pub fn new(width: usize, height: usize) -> CelluralMatrix {
        assert!(width != 0 && height != 0);
        let size = width.checked_mul(height).expect("too big");
        return CelluralMatrix {
            pixels: vec![Pixel::default(); size],
            width: width,
            height: height,
        }
    }

    fn set_pixel(&mut self, x: i64, y: i64, empty: bool) {
        if let Some(pixel) = self.pixels.iter_mut().find(|p| p.x == x && p.y == y) {
            pixel.empty = empty;
        } else {
            self.pixels.push(Pixel::new(x as i64, y as i64, empty ))
        }
    }

    fn get_bottom(&self, x: i64, y: i64) -> (bool, bool, bool) {
        let (xp1, xm1) = (x + 1, x - 1);
        let yp1 = y + 1;

        let pixel_below = self.pixels.iter().find(|p| p.x == x && p.y == yp1).map_or(true, |p| p.empty);
        let pixel_below_right = self.pixels.iter().find(|p| p.x == xp1 && p.y == yp1).map_or(true, |p| p.empty);
        let pixel_below_left = self.pixels.iter().find(|p| p.x == xm1 && p.y == yp1).map_or(true, |p| p.empty);

        return (pixel_below, pixel_below_right, pixel_below_left)
    }

    fn drop_sand(&mut self) {
        if let Some(index) = self.pixels.iter().position(|p| p.x == self.x && p.y == self.y) {
            while self.pixels[index].y < (self.height - 1).try_into().unwrap() {
                let (below, below_right, below_left) = self.get_bottom(self.pixels[index].x, self.pixels[index].y);
                if below {
                    self.pixels[index].y += 1;
                } else if below_right {
                    self.pixels[index].x += 1;
                    self.pixels[index].y += 1;
                } else if below_left {
                    self.pixels[index].x -= 1;
                    self.pixels[index].y += 1;
                } else {
                    break;
                }
            }
        }
    }

    fn draw(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), 4 * self.pixels.len());
        for (p, pix) in self.pixels.iter().zip(screen.chunks_exact_mut(4)) {
            let color = if p.empty {
                [STANDART_R, STANDART_G, STANDART_B, STANDART_A]
            } else {
                [0xff, 0xff, 0x00, 0xff]
            };
            pix.copy_from_slice(&color);
        }
    }
}

fn main() {
    let mut matrix = CelluralMatrix::new(61, 60);

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_inner_size(LogicalSize::new(800, 600)).build(&event_loop).unwrap();
    window.set_title("Sand Simulation");

    let surface_texture = SurfaceTexture::new(61,60, &window);

    let mut pixels = Pixels::new(61, 60, surface_texture).unwrap();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            Event::AboutToWait => {
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                pixels.resize_surface(800, 600).unwrap();

                let frame = pixels.frame_mut();

                matrix.set_pixel(31, 10, false);

                matrix.drop_sand();

                matrix.draw(frame);
                println!("Huy")
            },
            _ => ()
        }
    }).unwrap();
}