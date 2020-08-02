use specs::World;

use super::components::{
    create_box, create_box_spot, create_floor, create_player, create_wall, Position,
};

pub fn load_map(world: &mut World, map_string: String) {
    // read all lines
    let rows = map_string
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect::<Vec<_>>();

    for (y, row) in rows.iter().enumerate() {
        let columns = row.split(' ').collect::<Vec<_>>();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position::new2(x as u8, y as u8);

            // Figure out what object we should create
            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
  N N W W W W W W
  W W W . . . . W
  W . . . B . . W
  W . . . . . . W 
  W . P . . . . W
  W . . . . . . W
  W . . S . . . W
  W . . . . . . W
  W W W W W W W W
  ";

    load_map(world, MAP.to_string());
}
