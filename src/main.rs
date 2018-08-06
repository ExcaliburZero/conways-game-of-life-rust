extern crate gol;

fn main() {
    let rows = 20;
    let columns = rows;

    let mut board = gol::Board::new(rows, columns);

    board.place_pattern(gol::Glider {}, 1, 2);
    board.place_pattern(gol::Blinker {}, 10, 10);

    let image_scaling = 4;
    let generations = 50;

    for gen in 0..generations {
        board.next_generation();

        let filepath = get_image_path(gen);

        println!("{}", filepath);
        board.to_image(image_scaling)
            .save(&filepath).unwrap();
    }
}

fn get_image_path(generation: i32) -> String {
    let mut filepath = String::new();
    filepath.push_str("img/");
    filepath.push_str(&generation.to_string());
    filepath.push_str(".png");

    filepath
}
