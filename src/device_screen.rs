use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, /*ControlFlow,*/ EventLoop},
    window::Window,
};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 150;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct DeviceScreen {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
}

impl DeviceScreen {
    pub fn new() -> Self {
        Self {
            window: None,
            pixels: None,
        }
    }

    pub fn run() -> Result<()> {
        let event_loop = EventLoop::new()?;
        let mut app = DeviceScreen::default();
        event_loop.run_app(&mut app)?;

        Ok(())
    }
}

impl ApplicationHandler for DeviceScreen {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("emoboy"))
            .unwrap();

        let size = window.inner_size();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);

        let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

        self.window = Some(window_ref);
        self.pixels = Some(pixels);

        let scale_factor = window_ref.scale_factor();
        println!("Scale factor : {}", scale_factor);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    // Remplir tout en bleu
                    for spot in frame.chunks_exact_mut(4) {
                        spot[0] = 0x20; // R
                        spot[1] = 0x40; // G
                        spot[2] = 0xFF; // B
                        spot[3] = 0xFF; // A
                    }

                    pixels.render().unwrap();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window
            .expect("Bug - Window should exist")
            .request_redraw();
    }
}
