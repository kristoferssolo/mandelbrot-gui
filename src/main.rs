use eframe::{App, Error, NativeOptions};
use egui::{CentralPanel, Color32, ColorImage, Image, PointerButton, Pos2, Vec2, ViewportBuilder};
use image::{ImageBuffer, Rgb};
use num_complex::Complex;

const MAX_ITER: u32 = 256;

fn mandelbrot(c: Complex<f64>) -> u32 {
    let mut z = Complex::new(0., 0.);
    for i in 0..MAX_ITER {
        if z.norm_sqr() > 4. {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}

struct MandelbrotApp {
    image: ColorImage,
    texture_handle: Option<egui::TextureHandle>,
    width: usize,
    height: usize,
    zoom: f64,
    center_x: f64,
    center_y: f64,
    dragging: bool,
    last_mouse_pos: Option<Pos2>,
}

impl MandelbrotApp {
    fn new(width: usize, height: usize) -> Self {
        Self {
            image: ColorImage::new([width, height], Color32::BLACK),
            texture_handle: None,
            width,
            height,
            zoom: 1.,
            center_x: -0.5,
            center_y: 0.,
            dragging: false,
            last_mouse_pos: None,
        }
    }

    fn generate_mandelbrot(&mut self) {
        let width = self.width;
        let height = self.height;
        let zoom = self.zoom;
        let center_x = self.center_x;
        let center_y = self.center_y;

        let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(width as u32, height as u32);

        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let x_val = center_x + (x as f64 - width as f64 / 2.0) / (width as f64 / 2.0) * zoom;
            let y_val = center_y + (y as f64 - height as f64 / 2.0) / (height as f64 / 2.0) * zoom;
            let c = Complex::new(x_val, y_val);

            let i = mandelbrot(c);
            let color_value = (i % 256) as u8;
            *pixel = Rgb([color_value, color_value, color_value]);
        }

        // Convert the `image` crate's `ImageBuffer` to `egui::ColorImage`
        let pixels = img_buf
            .into_raw()
            .chunks(3)
            .map(|chunk| Color32::from_rgb(chunk[0], chunk[1], chunk[2]))
            .collect::<Vec<_>>();
        self.image.pixels = pixels;
    }
}

impl App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Solo Mandelbrot Set");

            // Handle mouse wheel for zooming
            let scroll_delta = ctx.input(|i| i.raw_scroll_delta);
            if scroll_delta.y != 0.0 {
                let zoom_factor = 1.1;
                if scroll_delta.y > 0.0 {
                    self.zoom /= zoom_factor; // Zoom in
                } else {
                    self.zoom *= zoom_factor; // Zoom out
                }
                self.generate_mandelbrot();
                self.texture_handle = None; // Invalidate the texture
            }

            // Handle mouse dragging for panning
            if ui.input(|i| i.pointer.button_down(PointerButton::Primary)) {
                if let Some(pos) = ctx.pointer_interact_pos() {
                    dbg!(&pos);
                    if self.dragging {
                        if let Some(last_pos) = self.last_mouse_pos {
                            let delta_x = (pos.x - last_pos.x) as f64 / self.zoom;
                            let delta_y = (pos.y - last_pos.y) as f64 / self.zoom;
                            self.center_x -= delta_x;
                            self.center_y -= delta_y;
                            self.generate_mandelbrot();
                            self.texture_handle = None; // Invalidate the texture
                        }
                    }
                    self.last_mouse_pos = Some(pos);
                    self.dragging = true;
                }
            } else {
                self.dragging = false;
                self.last_mouse_pos = None;
            }

            // Load the texture only if it's not already loaded or if the image has changed
            if self.texture_handle.is_none() {
                self.texture_handle =
                    Some(ctx.load_texture("mandelbrot", self.image.clone(), Default::default()));
            }

            // Display the image
            if let Some(texture_handle) = &self.texture_handle {
                let pixels_per_point = ctx.pixels_per_point();
                ui.add(Image::new(texture_handle).fit_to_original_size(pixels_per_point));
            }
        });
    }
}

fn main() -> Result<(), Error> {
    let width = 800;
    let height = 600;

    let mut app = MandelbrotApp::new(width, height);
    app.generate_mandelbrot();

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2::new(width as f32, height as f32)),
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        ..Default::default()
    };

    eframe::run_native(
        "Solo Mandelbrot Set",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
