extern crate gol;

fn main() {
    let height = 9;
    let width = height;

    let dims = (height, width);
    let mut board = gol::Board::new(dims);

    board.place_pattern(gol::Glider {}, 1, 2);

    let generations = 5;

    println!("{}", board);
    for _gen in 0..generations {
        board.next_generation();

        println!("{}", board);
    }
}
