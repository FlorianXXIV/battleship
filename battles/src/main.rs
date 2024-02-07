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

    enemies[1].position[1] = (1, -2, 1);
    enemies[1].position[1] = (2, -2, 0);

    (player, enemies)
}