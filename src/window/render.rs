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
    pub fn draw(&self) {
        unimplemented!();
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
        let gl_y = board_y as f32 / view.cells_on_height as f32 * 2.0 - 1.0;
        let gl_x = board_x as f32 / view.cells_on_width as f32 * 2.0 - 1.0;

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
    super::graphics_context::GraphicsContext::clear_color(color[0], color[1], color[2], color[3]);
}

/// Renders the board
///
/// **NOTE**: Does not update the board as well.
pub fn render(
    config: &super::Config,
    view: &super::View,
    board: Vec<Vec<super::game_of_life::CellState>>,
    alive_color: [f32; 4],
    dead_color: [f32; 4],
) {
    clear_screen(dead_color);

    for board_y in 0..view.cells_on_height {
        for board_x in 0..view.cells_on_width {
            if board[board_y + view.y][board_x + view.x] == super::game_of_life::CellState::Alive {
                let square = Square::simple(view, board_x, board_y, alive_color);
                square.draw();
            } else if config.view_border
                && (board_y + view.y == 0
                    || board_y + view.y + 1 == view.board_height
                    || board_x + view.x == 0
                    || board_x + view.x + 1 == view.board_width)
            {
                let square = Square::simple(view, board_x, board_y, [0.5, 0.5, 0.5, 1.0]);
                square.draw();
            }
        }
    }
}
