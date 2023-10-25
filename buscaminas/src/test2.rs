fn main(){
    let mut bombs: Vec<Vec<u8>> = vec![vec![3; 4]; 4];
    for row in &bombs {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}