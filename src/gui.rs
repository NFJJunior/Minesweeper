use druid::widget::{Align, Flex, Label};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

static mut CELL_SIZE: f64 = 40.0;  //  Size of a cell for interface
static mut GRID_SIZE: f64 = 9.0;  //  Length of the map matrice

use super::Map;

//  Build the interface for the popup end game screen
unsafe fn build_popun(game_won: bool) -> impl Widget<Map> {
    let label: Label<Map>;

    if game_won {
        label = Label::new("You won!");
    } else {
        label = Label::new("You lost!");
    }

    let label = Align::centered(label);
    let label = label.fix_size(200.0, 200.0);

    label
}

//  Build the interface for the game
//  The interface is a flex container with as many childs as cells in the map
//  Every child has a label which value is determined by the print function from the Map class
//  What the player:
//      '-' - a cell that hasn't been touched yet
//      '⚑' - a flaged cell
//      '*' - a mined cell
//      ' ' - a cell with no mines near by
//      'x' - a cell with x mines near by
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
                    //  If the game didn't end already, the first time you click
                    //  on a cell, it flags it, than the second time you click
                    //  on it, it reveals it
                    //  If the revealed cell is a mined cell, you lost the game.
                    //  If the game has ended, the popup window will show
                    if !data.is_ended {
                        if data.playing_map[i][j] == 0 {
                            data.flag(i as u32, j as u32);
                        } else if data.playing_map[i][j] == 1 {
                            if data.reveal(i as u32, j as u32) {
                                if data.game_won() {
                                    let window = WindowDesc::new(build_popun(true))
                                        .window_size((200.0, 200.0))
                                        .title("Ending screen");

                                    ctx.new_window(window);
                                }
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

    //  Create the main window with the interface we built, a given size and a title
    let main_window = WindowDesc::new(build_ui())
        .window_size((GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE))
        .title("Minesweeper");

    //  Launch the app
    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
