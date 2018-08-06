extern crate gol;

fn main() {
    let rows = 100;
    let columns = rows;

    let mut board = gol::Board::new(rows, columns);

    board.place_pattern(gol::Glider {}, 1, 2);

    let generations = 300;

    for gen in 0..generations {
        board.next_generation();

        let filepath = get_image_path(gen);

        println!("{}", filepath);
        board.to_image(&filepath).unwrap();
    }
}

fn get_image_path(generation: i32) -> String {
    let mut filepath = String::new();
    filepath.push_str("img/");
    filepath.push_str(&generation.to_string());
    filepath.push_str(".png");

    filepath
}
