use druid::widget::{Align, Flex, Label};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

static mut CELL_SIZE: f64 = 40.0;
static mut GRID_SIZE: f64 = 9.0;

use super::Map;

unsafe fn build_popun(game_won: bool) -> impl Widget<Map> {
    let label: Label<Map>;

    if game_won {
        label = Label::new("You won!");
    } else {
        label = Label::new("You lost");
    }

    let label = Align::centered(label);
    let label = label.fix_size(200.0, 200.0);

    label
}

unsafe fn build_ui() -> impl Widget<Map> {
    let mut grid: Flex<Map> = Flex::column();

    unsafe {
        for i in 0..GRID_SIZE as usize {
            let mut row: Flex<Map> = Flex::row();

            for j in 0..GRID_SIZE as usize {
                let label: Label<Map> =
                    Label::new(move |data: &Map, _env: &_| data.print(i as u32, j as u32));

                let cell = Align::centered(label);
                let cell = cell.border(Color::grey(0.6), 1.0);
                let cell = cell.on_click(move |ctx, data: &mut Map, _env| {
                    if data.is_lost {
                        if data.playing_map[i][j] == 0 {
                            data.flag(i as u32, j as u32);
                        } else if data.playing_map[i][j] == 1 {
                            if data.reveal(i as u32, j as u32) {
                                data.game_won();
                            } else {
                                data.game_lost();

                                let window = WindowDesc::new(build_popun(false))
                                    .window_size((200.0, 200.0))
                                    .title("Ending screen");

                                ctx.new_window(window);
                            }
                        }
                    }
                });

                row.add_flex_child(cell, 1.0);
            }

            let row = row.fix_size(GRID_SIZE * CELL_SIZE, CELL_SIZE);
            grid.add_flex_child(row, 1.0);
        }
    }

    let grid = grid.fix_size(GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE);

    grid
}

pub unsafe fn main(level: super::LEVEL) {
    let initial_data = Map::new(level);
    GRID_SIZE = initial_data.map_size as f64;

    let main_window = WindowDesc::new(build_ui())
        .window_size((GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE))
        .title("Minesweeper");

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
