fn modify_player_view(player_view: &mut Vec<Vec<char>>, x: usize, y: usize, bombs: &Vec<Vec<i32>>, keep_playing: &mut bool){
    println!("Value at coordinates ({}, {}): {}", x, y, bombs[x][y]);
    if bombs[x][y] == 1 {
        println!("BOOM!");
        player_view[x][y] = '!';
        *keep_playing = false;
    } else {
        player_view[x][y] = 'o';
    }
}

fn main() {
    let mut player_view: Vec<Vec<char>> = vec![
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
    ];

    let bombs: Vec<Vec<i32>> = vec![
        vec![0, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
    ];

    for i in &player_view {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }

    
    
    let mut keep_playing: bool = true;

    while keep_playing {
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

        let numero1: usize = numero1.trim().parse().expect("Please enter a valid number");
        let numero2: usize = numero2.trim().parse().expect("Please enter a valid number");

        // Using get method to handle out-of-bounds gracefully
        if let Some(row) = bombs.get(numero1) {
            // Using get method again for the inner vector
            if let Some(_cell) = row.get(numero2) {
                modify_player_view(&mut player_view, numero1, numero2, &bombs, &mut keep_playing);
            } else {
                println!("Invalid coordinate Y");
            }
        } else {
            println!("Invalid coordinate X");
        }

        for i in &player_view {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }
    }
}
