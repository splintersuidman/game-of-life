use super::Config;

pub struct View {
    pub cell_width: f32,
    base_cell_width: f32,

    precise_y: f64,
    precise_x: f64,

    capture_cursor: bool,

    pub cells_on_width: usize,
    pub cells_on_height: usize,

    pub board_width: usize,
    pub board_height: usize,

    pub window_width: f32,
    pub window_height: f32,
}

impl View {
    /// Retrieves the rounded x coordinate.
    pub fn round_x(&self) -> usize {
        self.precise_x.floor() as usize
    }

    /// Retrieves the rounded y coordinate.
    pub fn round_y(&self) -> usize {
        self.precise_y.floor() as usize
    }

    /// Returns a bool indicating whether or not the cursor should be captured.
    pub fn capture_cursor(&self) -> bool {
        self.capture_cursor
    }

    /// Checks whether it's neccessary to be able to move around the board.
    pub fn check_capture_cursor(&mut self) {
        if self.board_width as f32 * self.cell_width > self.window_width
            || self.board_height as f32 * self.cell_width > self.window_height
        {
            self.capture_cursor = true;
        }
    }

    /// Toggles whether the cursor should be captured or not.
    pub fn toggle_capture_cursor(&mut self) {
        self.capture_cursor = !self.capture_cursor;
    }

    /// Returns the cell_width in the correct proportions for OpenGL.
    pub fn gl_cell_width(&self) -> f32 {
        self.cell_width / self.window_width as f32 * 2.0
    }

    /// Returns the cell_height in the correct proportions for OpenGL.
    pub fn gl_cell_height(&self) -> f32 {
        self.cell_width / self.window_height as f32 * 2.0
    }

    /// Guesses the ideal window size for a board
    ///
    /// Uses the screen size, the board size, and cell_width to make its guess.
    /// Also captures the cursor if neccessary with `check_capture_cursor()` and makes sure everything is alright via `on_resize()`.
    /// Intended only for use on startup.
    pub fn determine_window_size(&mut self, screen_width: f32, screen_height: f32) {
        self.window_width = (self.board_width as f32 * self.cell_width).min(screen_width);
        self.window_height = (self.board_height as f32 * self.cell_width).min(screen_height);

        let (width, height) = (self.window_width, self.window_height);

        self.on_resize(width, height);

        self.check_capture_cursor();
    }

    /// Initialises a `View` struct based on the given settings.
    ///
    /// **NOTE:** Because the screen size is still unknown the struct is not ready to be used yet;
    /// call the `determine_window_size()` function first.
    pub fn from_config(config: &Config) -> Self {
        let board_width = config.width as usize;
        let board_height = config.height as usize;

        let base_cell_width: f32 = config.cell_width as f32;
        let cell_width = base_cell_width;

        let window_width = 0.0;
        let window_height = 0.0;

        let cells_on_width = (window_width as f32 / cell_width).ceil() as usize;
        let cells_on_height = (window_height as f32 / cell_width).ceil() as usize;

        Self {
            precise_y: 0.0,
            precise_x: 0.0,

            cell_width,
            base_cell_width,

            capture_cursor: false,

            window_width,
            window_height,

            cells_on_width,
            cells_on_height,

            board_width,
            board_height,
        }
    }

    /// Chooses the correct `cell_width` for the new window size.
    ///
    /// Makes sure `cell_width`, `window_width`, `window_height`, `cells_on_width`, and `cells_on_height` are proportionate to one another.
    /// Strives for using `base_cell_width` but adapts if that's not possible.
    ///
    /// **NOTE:** Does not change `base_cell_width`.
    pub fn on_resize(&mut self, width: f32, height: f32) {
        self.window_width = width;
        self.window_height = height;

        // Reset to the base cell width.
        self.cell_width = self.base_cell_width;

        self.cells_on_width = (self.window_width / self.cell_width).ceil() as usize;
        self.cells_on_height = (self.window_height / self.cell_width).ceil() as usize;

        if self.cells_on_width > self.board_width {
            self.cell_width = self.window_width / self.board_width as f32;
            self.cells_on_width = (self.window_width / self.cell_width) as usize;
            self.cells_on_height = (self.window_height / self.cell_width) as usize;
        }

        if self.cells_on_height > self.board_height {
            self.cell_width = self.window_height / self.board_height as f32;
            self.cells_on_width = (self.window_width / self.cell_width) as usize;
            self.cells_on_height = (self.window_height / self.cell_width) as usize;
        }

        // Trigger function to check for moving outside of the board.
        self.on_mouse_move(0.0, 0.0);
    }

    /// Moves the view around if appropriate.
    pub fn on_mouse_move(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.capture_cursor {
            // Prevent y from moving outside of the board and update it.
            self.precise_y = (self.precise_y - mouse_y)
                .max(0.0)
                .min((self.board_height - self.cells_on_height) as f64);

            // Prevent x from moving outside of the board and update it.
            self.precise_x = (self.precise_x - mouse_x)
                .max(0.0)
                .min((self.board_width - self.cells_on_width) as f64);
        }
    }

    // pub fn get_center(&self) -> (f32, f32) {
    //     let x = self.cells_on_width as f32 / 2.0 + self.precise_x as f32;
    //     let y = self.cells_on_height as f32 / 2.0 + self.precise_y as f32;

    //     (x, y)
    // }

    // pub fn set_center(&mut self, coordinates: (f32, f32)) {
    //     let (x, y) = coordinates;
    //     let (x, y) = (x as f64, y as f64);

    //     self.precise_x = x - self.cells_on_width as f64 / 2.0;
    //     self.precise_y = y - self.cells_on_height as f64 / 2.0;

    //     self.y = self.precise_y as usize;
    //     self.x = self.precise_x as usize;
    // }

    /// Zooms in and out if appropriate.
    pub fn on_scroll(&mut self, y: f32) {
        // let center = self.get_center();
        self.base_cell_width += self.base_cell_width * y * 0.01;

        // Check if zooming further out is possible
        if (self.base_cell_width * self.board_width as f32) < self.window_width {
            self.base_cell_width = self.window_width / self.board_width as f32;
        }
        if (self.base_cell_width * self.board_height as f32) < self.window_height {
            self.base_cell_width = self.window_height / self.board_height as f32;
        }

        // Check if zooming further in is possible
        self.base_cell_width = self
            .base_cell_width
            .min(self.window_width)
            .min(self.window_height);

        let width = self.window_width;
        let height = self.window_height;

        self.on_resize(width, height);

        // self.set_center(center);
    }
}
