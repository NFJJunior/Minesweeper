use druid::widget::{Align, Flex, Label};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, Color};

const CELL_SIZE: f64 = 40.0;
const GRID_SIZE: f64 = 9.0;

use super::Map;

fn build_ui() -> impl Widget<Map> {
    let mut grid: Flex<Map> = Flex::column();

    for i in 0..GRID_SIZE as usize {
        let mut row: Flex<Map> = Flex::row();

        for j in 0..GRID_SIZE as usize {
            let label: Label<Map> = Label::new(move |data: &Map, _env: &_| {

                data.print(i as u32, j as u32)
            });

            let cell = Align::centered(label);
            let cell = cell.border(Color::grey(0.6), 1.0);
            let cell = cell.on_click(move |_ctx, data: &mut Map, _env| {
                if data.playing_map[i][j] == 0 {
                    data.flag(i as u32, j as u32);
                }
                else if data.playing_map[i][j] == 1 {
                    data.reveal(i as u32, j as u32);
                } 
            });

            row.add_flex_child(cell, 1.0);
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

    let initial_data = Map::new(super::LEVEL::EASY);

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
