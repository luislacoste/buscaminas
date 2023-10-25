use rand::Rng;

fn modify_player_view(player_view: &mut Vec<Vec<char>>,x: usize,y: usize,bombs: &Vec<Vec<u8>>,keep_playing: &mut bool) {
    if bombs[x][y] == 8 {
        player_view[x][y] = '!';
        print!("{}[2J", 27 as char);

        for i in player_view.iter() {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }
        println!("BOOM!");
        *keep_playing = false;
    } else {
        player_view[x][y] = bombs[x][y].to_string().chars().next().unwrap();
        print!("{}[2J", 27 as char);
    }
}

fn fill_bombs(bombs: &mut Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();
    let mut counter: u8 = 0;

    for i in bombs {
        for j in i {
            let random_number = rng.gen_range(0..=1);
            if random_number == 1 && counter < 2 {
                counter += 1;
                *j = 8;
            } else {
                *j = 0;
            }
        }
    }
}

fn fill_nearby_bombs(bombs: &mut Vec<Vec<u8>>) {
    // Iterate over rows
    for row in 0..bombs.len() {
        // Iterate over columns
        for column in 0..bombs[row].len() {
            // Check if the current element is 8
            if bombs[row][column] == 8 {
                // Top
                if row > 0 && bombs[row - 1][column] != 8 {
                    bombs[row - 1][column] += 1;
                }
                // Bottom
                if row < bombs.len() - 1 && bombs[row + 1][column] != 8 {
                    bombs[row + 1][column] += 1;
                }
                // Left
                if column > 0 && bombs[row][column - 1] != 8 {
                    bombs[row][column - 1] += 1;
                }
                // Right
                if column < bombs[row].len() - 1 && bombs[row][column + 1] != 8 {
                    bombs[row][column + 1] += 1;
                }
                // Top-left
                if row > 0 && column > 0 && bombs[row - 1][column - 1] != 8 {
                    bombs[row - 1][column - 1] += 1;
                }
                // Top-right
                if row > 0 && column < bombs[row].len() - 1 && bombs[row - 1][column + 1] != 8 {
                    bombs[row - 1][column + 1] += 1;
                }
                // Bottom-left
                if row < bombs.len() - 1 && column > 0 && bombs[row + 1][column - 1] != 8 {
                    bombs[row + 1][column - 1] += 1;
                }
                // Bottom-right
                if row < bombs.len() - 1 && column < bombs[row].len() - 1 && bombs[row + 1][column + 1] != 8 {
                    bombs[row + 1][column + 1] += 1;
                }
            }
        }
    }
}

// fn create_field_player(player_view: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
//     for i in 0..player_view.len() {
//         for j in 0..player_view[i].len() {
//             player_view[i][j] = 'x';
//         }
//     }
// }

fn main() {
    // let mut player_view: Vec<Vec<char>> = vec![
    //     vec!['x', 'x', 'x', 'x'],
    //     vec!['x', 'x', 'x', 'x'],
    //     vec!['x', 'x', 'x', 'x'],
    //     vec!['x', 'x', 'x', 'x'],
    // ];

    // let mut bombs: Vec<Vec<u8>> = vec![vec![0; 4]; 4];

    // TODO validate size
    let mut size = String::new();
    println!("Ingrese el tamaño del tablero: ");
    std::io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    let size: usize = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid size");
            4
        }
    };

    let mut player_view: Vec<Vec<char>> = vec![vec!['x'; size]; size];
    // create_field_player(&mut player_view: Vec<Vec<char>>);

    let mut bombs: Vec<Vec<u8>> = vec![vec![0; size]; size];
    fill_bombs(&mut bombs);
    fill_nearby_bombs(&mut bombs);

    let mut keep_playing: bool = true;

    while keep_playing {
        for i in player_view.iter() {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }

        let mut numero1 = String::new();
        let mut numero2 = String::new();
        println!("Ingrese la coordenada X: ");
        std::io::stdin()
            .read_line(&mut numero1)
            .expect("Failed to read line");

        println!("Ingrese la coordenada Y: ");
        std::io::stdin()
            .read_line(&mut numero2)
            .expect("Failed to read line");

        let numero1: usize = match numero1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid coordinate X");
                continue;
            }
        };
        let numero2: usize = match numero2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid coordinate Y");
                continue;
            }
        };

        // Using get method to handle out-of-bounds gracefully
        if let Some(row) = bombs.get(numero1) {
            // Using get method again for the inner vector
            if let Some(_cell) = row.get(numero2) {
                modify_player_view(
                    &mut player_view,
                    numero1,
                    numero2,
                    &bombs,
                    &mut keep_playing,
                );
            } else {
                println!("Invalid coordinate Y");
            }
        } else {
            println!("Invalid coordinate X");
        }
    }
}
