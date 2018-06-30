extern crate cgmath;
extern crate clap;
extern crate game_of_life;
extern crate gl;
extern crate glutin;

mod config;
mod graphics_context;
mod render;
mod view;

use config::Config;
use game_of_life::GameOfLife;
use glutin::dpi::*;
use glutin::GlContext;
use render::Renderer;
use view::View;

fn main() {
    let config = Config::parse();

    let mut game_of_life = GameOfLife::new(config.width as usize, config.height as usize);

    if let Some(f) = config.file.clone() {
        game_of_life.init_with_file(f).unwrap();
    } else {
        game_of_life.init_randomly(config.chance);
    }

    let name = if let Some(ref n) = game_of_life.name {
        format!("Game of Life - {}", n)
    } else {
        String::from("Game of Life")
    };

    // Create View for managing boards larger than a window
    let mut view = View::from_config(&config);

    // Create window.
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(name)
        .with_dimensions(LogicalSize::new(600.0, 600.0));
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    let renderer = Renderer::new(&gl_window).unwrap();

    // Get the window.
    let size = gl_window.get_current_monitor().get_dimensions();
    let (screen_width, screen_height) = (size.width, size.height);

    println!(
        "Screen: width = {}px, height = {}px",
        screen_width, screen_height
    );

    view.determine_window_size(screen_width as f32, screen_height as f32);
    gl_window.set_inner_size(LogicalSize::new(
        view.window_width as f64,
        view.window_height as f64,
    ));

    // view.info();

    // TODO: grab cursor if board_width * cell_width > window_width.
    // gl_window.set_cursor_state(glutin::CursorState::Grab);

    // Set event loop settings
    // let mut settings = window.get_event_settings();
    // settings.set_ups(0);
    // settings.set_max_fps(config.fps);
    // window.set_event_settings(settings);

    let mut closed = false;
    while !closed {
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => {
                    // Window was closed.
                    closed = true;
                }
                glutin::WindowEvent::Resized(size) => {
                    // Window was resized.
                    view.on_resize(size.width as f32, size.height as f32);
                }
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    // Left-mouse-button pressed.
                    if state == glutin::ElementState::Pressed && glutin::MouseButton::Left == button
                    {
                        // reinitialise board
                        if let Some(f) = config.file.clone() {
                            game_of_life.init_with_file(f).unwrap();
                        } else {
                            game_of_life.init_randomly(config.chance);
                        }
                    }
                }
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glutin::ElementState::Pressed
                        && input.virtual_keycode.is_some()
                    {
                        use glutin::VirtualKeyCode::*;

                        match input.virtual_keycode.unwrap() {
                            C => {
                                // Toggle capture_cursor.
                                view.toggle_capture_cursor();

                                if view.capture_cursor {
                                    gl_window.window().grab_cursor(true).unwrap();
                                    gl_window.window().hide_cursor(true);
                                } else {
                                    gl_window.window().grab_cursor(false).unwrap();
                                    gl_window.window().hide_cursor(false);
                                }
                            }
                            Escape => {
                                // Window has to close.
                                closed = true;
                            }
                            Space => {
                                // Reinitialise board.
                                if let Some(f) = config.file.clone() {
                                    game_of_life.init_with_file(f).unwrap();
                                } else {
                                    game_of_life.init_randomly(config.chance);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            glutin::Event::DeviceEvent { event, .. } => match event {
                glutin::DeviceEvent::MouseMotion { delta } => {
                    // Mouse moved.
                    view.on_mouse_move(delta.0, -delta.1);
                }
                _ => (),
            },
            _ => (),
        });

        renderer.render(&config, &view, &game_of_life);

        gl_window.swap_buffers().unwrap();

        game_of_life.update();
    }
}
