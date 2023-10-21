fn main() {
    let mut player_view: Vec<Vec<char>> = vec![
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x'],
        vec!['x', 'x', 'x', 'x']
    ];

    player_view[1][1] = 'o';

    for i in player_view {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}