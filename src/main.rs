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
use std::time::{Duration, Instant};
use view::View;

fn main() {
    let mut config = Config::parse();

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
        .with_dimensions(LogicalSize::new(
            (config.width * config.cell_width) as f64,
            (config.height * config.cell_width) as f64,
        ));
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    let renderer = Renderer::new(&gl_window).unwrap();

    // Get the window.
    let size = gl_window.get_current_monitor().get_dimensions();
    let (screen_width, screen_height) = (size.width, size.height);

    view.determine_window_size(screen_width as f32, screen_height as f32);
    gl_window.set_inner_size(LogicalSize::new(
        view.window_width as f64,
        view.window_height as f64,
    ));

    if view.capture_cursor {
        gl_window.window().grab_cursor(true).unwrap();
        gl_window.window().hide_cursor(true);
    }

    let delay = if config.fps == 0 {
        Duration::from_millis(0)
    } else {
        Duration::from_millis(((1.0 / config.fps as f32) * 1e3) as u64)
    };
    // Keep track of the previous time the board had been updated, to use the fps config variable.
    let mut previous_update = Instant::now() - delay;

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
                    let center = view.get_center();
                    view.on_resize(size.width as f32, size.height as f32);
                    view.set_center(center);

                    let dpi_factor = gl_window.get_hidpi_factor();
                    gl_window.resize(size.to_physical(dpi_factor));
                }
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    // Left-mouse-button pressed.
                    if state == glutin::ElementState::Pressed && glutin::MouseButton::Left == button
                    {
                        // Reinitialise board.
                        if let Some(f) = config.file.clone() {
                            game_of_life.init_with_file(f).unwrap();
                        } else {
                            game_of_life.init_randomly(config.chance);
                        }
                    }
                }
                glutin::WindowEvent::MouseWheel {
                    delta, modifiers, ..
                } => {
                    use glutin::MouseScrollDelta;
                    // Change scale when scrolling with ctrl.
                    if modifiers.ctrl {
                        match delta {
                            MouseScrollDelta::LineDelta(_x, y) => {
                                view.on_scroll(y);
                            }
                            MouseScrollDelta::PixelDelta(LogicalPosition { y, .. }) => {
                                view.on_scroll(y as f32);
                            }
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
                            F => {
                                // Use ctrl-cmd F for fullscreen.
                                if input.modifiers.ctrl && input.modifiers.logo {
                                    let monitor_id = gl_window.get_current_monitor();
                                    let size = monitor_id.get_dimensions();

                                    gl_window.window().set_fullscreen(Some(monitor_id));
                                    view.on_resize(size.width as f32, size.height as f32);
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
                glutin::WindowEvent::DroppedFile(f) => {
                    if let Some(f) = f.to_str() {
                        game_of_life.init_with_file(f).unwrap();
                        config.file = Some(f.to_string());
                    }
                }
                _ => (),
            },
            glutin::Event::DeviceEvent { event, .. } => {
                if let glutin::DeviceEvent::MouseMotion { delta } = event {
                    // Mouse moved.
                    view.on_mouse_move(delta.0, -delta.1);
                }
            }
            _ => (),
        });

        renderer.render(&config, &view, &game_of_life);
        gl_window.swap_buffers().unwrap();

        let now = Instant::now();
        if now.duration_since(previous_update) >= delay {
            game_of_life.update();
            previous_update = now;
        }
    }
}
