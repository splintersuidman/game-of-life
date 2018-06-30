use super::cgmath::{Matrix4, Vector3};
use super::graphics_context::GraphicsContext;

/// coordinates on the opengl field, ranging from -1 to 1
pub struct Coordinate {
    x: f32,
    y: f32,
}

pub struct Square {
    coordinates: [Coordinate; 4],
    color: [f32; 4],
}

impl Square {
    pub fn draw(&self, graphics_context: &GraphicsContext) {
        let width = self.coordinates[1].x - self.coordinates[0].x;
        let height = self.coordinates[0].y - self.coordinates[2].y;
        let scale = Matrix4::from_nonuniform_scale(width / 2.0, height / 2.0, 1.0);
        let translate = Matrix4::from_translation(Vector3::<f32>::new(
            self.coordinates[0].x + width / 2.0,
            self.coordinates[2].y + height / 2.0,
            0.0,
        ));
        // let translate = Matrix4::from_nonuniform_scale(
        // );
        graphics_context.draw_square_with_scale_translation(scale, translate);
        // unimplemented!();
    }

    /// Calculates the coordinates from the top-left corner, the width and the height
    pub fn new(width: f32, height: f32, x: f32, y: f32, color: [f32; 4]) -> Square {
        let top_left = Coordinate { x, y };
        let top_right = Coordinate { x: x + width, y };
        let bottom_left = Coordinate { x, y: y - height };
        let bottom_right = Coordinate {
            x: x + width,
            y: y - height,
        };

        Square {
            coordinates: [top_left, top_right, bottom_left, bottom_right],
            color,
        }
    }

    /// Takes a position on the board and calculates the coordinates
    pub fn simple(view: &super::View, board_x: usize, board_y: usize, color: [f32; 4]) -> Square {
        let gl_y = board_y as f32 / (view.cells_on_height + 2) as f32 * 2.0 - 1.0;
        let gl_x = board_x as f32 / (view.cells_on_width + 2) as f32 * 2.0 - 1.0;

        return Square::new(
            view.gl_cell_width(),
            view.gl_cell_height(),
            gl_x,
            gl_y,
            color,
        );
    }
}

pub fn clear_screen(color: [f32; 4]) {
    unimplemented!();
}

/// Renders the board
///
/// **NOTE**: Does not update the board as well.
pub fn render(
    config: &super::Config,
    view: &super::View,
    board: &Vec<Vec<super::game_of_life::CellState>>,
    graphics_context: &GraphicsContext,
) {
    clear_screen(config.background);

    for board_y in 0..view.cells_on_height {
        for board_x in 0..view.cells_on_width {
            if board[board_y + view.y][board_x + view.x] == super::game_of_life::CellState::Alive {
                let square = Square::simple(view, board_x, board_y, config.foreground);
                square.draw(graphics_context);
            } else if config.view_border
                && (board_y + view.y == 0
                    || board_y + view.y + 1 == view.board_height
                    || board_x + view.x == 0
                    || board_x + view.x + 1 == view.board_width)
            {
                let square = Square::simple(view, board_x, board_y, [0.5, 0.5, 0.5, 1.0]);
                square.draw(graphics_context);
            }
        }
    }
}
