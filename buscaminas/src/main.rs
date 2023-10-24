use rand::Rng;

fn modify_player_view(player_view: &mut Vec<Vec<char>>, x: usize, y: usize, bombs: &Vec<Vec<u8>>, keep_playing: &mut bool){
    if bombs[x][y] == 1 {
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
        player_view[x][y] = 'o';
        print!("{}[2J", 27 as char);
    }
}

fn fill_bombs(bombs: &mut Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();
    let mut counter: u8 = 0;

    for i in bombs {
        for j in i {
            let random_number = rng.gen_range(0..=1);
            if random_number == 1 && counter < 2{
                counter += 1;
                *j = 1;
            } else {
                *j = 0;
            }
        }
    }
}

fn main() {
    let mut player_view: Vec<Vec<char>> = vec![
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
    ];

    let mut bombs: Vec<Vec<u8>> = vec![vec![0; 4]; 4];
    
    fill_bombs(&mut bombs);

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
    }
}