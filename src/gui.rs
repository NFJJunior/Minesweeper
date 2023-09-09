use druid::widget::{Flex, Label};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

const CELL_SIZE: f64 = 40.0;
const GRID_SIZE: f64 = 9.0;

fn build_ui() -> impl Widget<()> {
    let mut grid: Flex<()> = Flex::column();

    for _ in 0..9 {
        let mut row: Flex<()> = Flex::row();

        for _ in 0..9 {
            let label = Label::new("1")
                .align_horizontal(druid::UnitPoint::new(0.5, 0.5))
                .align_vertical(druid::UnitPoint::new(0.5, 0.5));
            let label = label.fix_size(CELL_SIZE, CELL_SIZE);
            let label = label.border(Color::grey(0.6), 1.0);

            row.add_flex_child(label, 1.0);
        }

        let row = row.fix_size(GRID_SIZE * CELL_SIZE, CELL_SIZE);
        grid.add_flex_child(row, 1.0);
    }

    let grid = grid.fix_size(GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE);

    grid
}

pub fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE))
        .title("Minesweeper");

    let initial_data = ();

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
