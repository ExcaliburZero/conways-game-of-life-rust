extern crate gol;

fn main() {
    let rows = 9;
    let columns = rows;

    let mut board = gol::Board::new(rows, columns);

    board.place_pattern(gol::Glider {}, 1, 2);

    let generations = 5;

    println!("{}", board);
    for _gen in 0..generations {
        board.next_generation();

        println!("{}", board);
    }
}
