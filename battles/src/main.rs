type Damage = u32;

struct Ship {
    hull: Vec<u32>,
    fuel: u32,
    weapons: Vec<Damage>,
    /// The Position is split into three axees
    /// (West, Horizon, East)
    /// All three numbers must add up to 0
    /// since this is a cubic coordinate system for a hexagonal grid.
    position: Vec<(i32, i32, i32)>
}
enum Tile {
    Land,
    Water
}

fn main() {
    let board = init_battle();
    game_loop(board.0, board.1);
}


fn init_battle() -> (Ship, Vec<Ship>) {
    let player = Ship {
        hull: vec![100, 100, 100],
        fuel: 100,
        weapons: vec![25, 25, 50],
        position: vec![(-1, 0, 1), (0, 0, 0), (1, 0, -1)],
    };

    let mut enemies: Vec<Ship> = Vec::new();
    for i in ..2 {
        enemies.push(
            Ship{
                hull: vec![100, 100],
                fuel: 100,
                weapons: vec![25],
                position: vec![(0, 0, 0), (0, 0, 0)],
            }
        )
    }

    enemies[0].position[0] = (-3, 3, 0);
    enemies[0].position[1] = (-3, 2, 1);

    enemies[1].position[0] = (1, -2, 1);
    enemies[1].position[1] = (2, -2, 0);

    (player, enemies)
}

fn game_loop(player: Ship, enemies: Vec<Ship>) {
    let round = 0;
    let mut game_over = false;
    loop {
        if round % enemies.len()+1 == 0 {
            game_over = prompt_player();
        } else {
            game_over = enemy_turn(round % enemies.len());
        }
        if game_over {
            break;
        }
    }
}

fn enemy_turn(p0: usize) -> bool {
    todo!()
}

fn prompt_player() -> bool {
    todo!()
}

fn fire(west: i32, horizon: i32, east: i32, ships: Vec<&mut Ship>, damage: Damage) -> i32{
    let mut pos = -1;
    for mut ship in ships {
        pos +=1;
        let mut i = 0;
        let mut health = 0;
        for position in ship.position{
            i += 1;
            if (west, horizon, east) == position {
                if ship.hull[i] - damage < 0 {
                    ship.hull [i] = 0;
                    ship.weapons.pop();
                }else {
                    ship.hull[i] -= damage;
                }
            }
        }
        for hull in ship.hull {
            health += hull;
        }
        if health == 0 {
            pos
        }
    }
    -1
}
