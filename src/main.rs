use stratego_rs::Game;
use stratego_rs::Piece;
use stratego_rs::Position;
use stratego_rs::Rank;

fn main() {
    // Example usage
    let mut game = Game::new();
    let marshal = Piece::new(Rank::Marshal);
    let spy = Piece::new(Rank::Spy);
    let pos_marshal = Position::new(0, 0);
    let pos_spy = Position::new(1, 0);

    game.add_piece(marshal, pos_marshal);
    game.add_piece(spy, pos_spy);

    println!("Game state: {:?}", game);

    // Get references to the pieces in the game
    if let Some(piece_marshal) = game.board.get(&pos_marshal) {
        println!("Created marshal: {:?}", piece_marshal);
        println!("Marshal's rank: {:?}", piece_marshal.rank);
    }
    if let Some(piece_spy) = game.board.get(&pos_spy) {
        println!("Created spy: {:?}", piece_spy);
        println!("Spy's rank: {:?}", piece_spy.rank);
    }
}
