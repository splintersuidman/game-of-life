use super::Config;

pub struct View {
    pub cell_width: f64,
    base_cell_width: f64,

    pub y: usize,
    pub x: usize,

    precise_y: f64,
    precise_x: f64,

    pub capture_cursor: bool,

    pub cells_on_width: usize,
    pub cells_on_height: usize,

    pub board_width: usize,
    pub board_height: usize,

    pub window_width: u32,
    pub window_height: u32,
}

impl View {
    pub fn toggle_capture_cursor(&mut self) {
        self.capture_cursor = !self.capture_cursor;
    }

    pub fn gl_cell_width(&self) -> f64 {
        return self.cell_width / self.window_width as f64 * 2.0;
    }

    pub fn gl_cell_height(&self) -> f64 {
        return self.cell_width / self.window_height as f64 * 2.0;
    }

    pub fn determine_window_size(&mut self, screen_width: u32, screen_height: u32) {
        self.window_width = if self.board_width as f64 * self.cell_width > screen_width as f64 {
            screen_width
        } else {
            (self.board_width as f64 * self.cell_width) as u32
        };
        self.window_height = if self.board_height as f64 * self.cell_width > screen_height as f64 {
            screen_height
        } else {
            (self.board_height as f64 * self.cell_width) as u32
        };

        let (width, height) = (self.window_width, self.window_height);

        self.on_resize(width, height);
    }

    pub fn from_config(config: &Config) -> Self {
        let board_width = config.width as usize;
        let board_height = config.height as usize;

        let base_cell_width: f64 = config.cell_width.into();
        let cell_width = base_cell_width.clone();

        let window_width = 0;
        let window_height = 0;

        let cells_on_width = (window_width as f64 / cell_width) as usize;
        let cells_on_height = (window_height as f64 / cell_width) as usize;

        Self {
            y: 0,
            x: 0,

            precise_y: 0.0,
            precise_x: 0.0,

            cell_width,
            base_cell_width,

            capture_cursor: true,

            window_width,
            window_height,

            cells_on_width,
            cells_on_height,

            board_width,
            board_height,
        }
    }

    pub fn on_resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;

        // reset to the base cell width
        self.cell_width = self.base_cell_width;

        self.cells_on_width = (self.window_width as f64 / self.cell_width) as usize;
        self.cells_on_height = (self.window_height as f64 / self.cell_width) as usize;

        if self.cells_on_width > self.board_width {
            self.cell_width = self.window_width as f64 / self.board_width as f64;
            self.cells_on_width = (self.window_width as f64 / self.cell_width) as usize;
            self.cells_on_height = (self.window_height as f64 / self.cell_width) as usize;
        }

        if self.cells_on_height > self.board_height {
            self.cell_width = self.window_height as f64 / self.board_height as f64;
            self.cells_on_height = (self.window_height as f64 / self.cell_width) as usize;
        }

        // trigger function to check for moving outside of the board
        self.on_mouse_move(0.0, 0.0);
    }

    pub fn on_mouse_move(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.capture_cursor {
            // prevent y from moving outside of the board and update it
            if self.precise_y - mouse_y < 0.0 {
                self.precise_y = 0.0;
            } else if self.precise_y - mouse_y + self.cells_on_height as f64
                > self.board_height as f64
            {
                self.precise_y = (self.board_height - self.cells_on_height) as f64;
            } else {
                self.precise_y -= mouse_y;
            }

            // prevent x from moving outside of the board and update it
            if self.precise_x - mouse_x < 0.0 {
                self.precise_x = 0.0;
            } else if self.precise_x - mouse_x + self.cells_on_width as f64
                > self.board_width as f64
            {
                self.precise_x = (self.board_width - self.cells_on_width) as f64;
            } else {
                self.precise_x -= mouse_x;
            }

            self.y = self.precise_y as usize;
            self.x = self.precise_x as usize;
        }
    }
}
