use super::cgmath::{Matrix4, Vector3};
use super::game_of_life::{CellState, GameOfLife};
use super::glutin;
use super::graphics_context::GraphicsContext;
use super::view::View;
use super::Config;

/// coordinates on the opengl field, ranging from -1 to 1
pub struct Coordinate {
    x: f32,
    y: f32,
}

pub struct Square {
    coordinates: [Coordinate; 4],
}

impl Square {
    /// Calculates the coordinates from the top-left corner, the width and the height
    pub fn new(width: f32, height: f32, x: f32, y: f32) -> Self {
        let top_left = Coordinate { x, y };
        let top_right = Coordinate { x: x + width, y };
        let bottom_left = Coordinate { x, y: y - height };
        let bottom_right = Coordinate {
            x: x + width,
            y: y - height,
        };

        Square {
            coordinates: [top_left, top_right, bottom_left, bottom_right],
        }
    }

    /// Takes a position on the board and calculates the coordinates
    pub fn simple(view: &super::View, board_x: usize, board_y: usize) -> Self {
        let gl_y = board_y as f32 / view.cells_on_height as f32 * 2.0 - 1.0;
        let gl_x = board_x as f32 / view.cells_on_width as f32 * 2.0 - 1.0;

        return Square::new(view.gl_cell_width(), view.gl_cell_height(), gl_x, gl_y);
    }
}

pub struct Renderer {
    pub graphics_context: GraphicsContext,
}

impl Renderer {
    pub fn new(gl_window: &glutin::GlWindow) -> Result<Self, String> {
        let mut graphics_context = GraphicsContext::new();
        graphics_context.init(gl_window)?;
        Ok(Renderer { graphics_context })
    }

    pub fn render(&self, config: &Config, view: &View, game_of_life: &GameOfLife) {
        self.clear_screen(config.foreground);

        for board_y in 0..view.cells_on_height {
            for board_x in 0..view.cells_on_width {
                if game_of_life.board[board_y + view.y][board_x + view.x] == CellState::Alive {
                    let square = Square::simple(view, board_x, board_y);
                    self.draw_square(&square);
                } else if config.view_border
                    && (board_y + view.y == 0
                        || board_y + view.y + 1 == view.board_height
                        || board_x + view.x == 0
                        || board_x + view.x + 1 == view.board_width)
                {
                    let square = Square::simple(view, board_x, board_y);
                    self.draw_square(&square);
                }
            }
        }
    }

    fn clear_screen(&self, color: [f32; 4]) {
        GraphicsContext::clear_color(color[0], color[1], color[2], color[3]);
    }

    fn draw_square(&self, square: &Square) {
        let width = square.coordinates[1].x - square.coordinates[0].x;
        let height = square.coordinates[0].y - square.coordinates[2].y;
        let scale = Matrix4::from_nonuniform_scale(width / 2.0, height / 2.0, 1.0);
        let translate = Matrix4::from_translation(Vector3::<f32>::new(
            square.coordinates[0].x + width / 2.0,
            square.coordinates[2].y + height / 2.0,
            0.0,
        ));
        self.graphics_context
            .draw_square_with_scale_translation(scale, translate);
    }
}
