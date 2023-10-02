use crate::board::*;
use crate::move_engine::*;

use clap::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute the best move
    Bestmove(EngineArgs),
}

#[derive(Args)]
struct EngineArgs {
    /// Max time spent calculating
    #[arg(short, long, default_value_t = 3)]
    time: u64,

    /// Transpositin table entrys
    #[arg(short, long, default_value_t = 100_000)]
    size: usize,

    /// Game position
    #[arg(short, long, default_value_t = String::from(""))]
    position: String,
}

pub fn app() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bestmove(args) => {
            let mut e = Engine::new(args.time, args.size);
            let b = Board::from_position(&args.position);
            let m = e.iterative_depening(&b);
            println!("{:?}", m);
        }
    }
    /*let mut board = board::Board::new();
    let mut e = move_engine::Engine::new(3, 100_000);

    println!("p1 for first player, p2 for second");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let human = match buf.trim() {
        "p1" => board::Player::P1,
        "p2" => board::Player::P2,
        _ => panic!("player option not valid"),
    };

    while board.gamestate() == board::GameState::Open {
        if board.player() == human {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let col: u8 = buf.trim().parse().unwrap();

            board.make_move(col);
        } else {
            let m = e.iterative_depening(&board);
            board.make_move(m.col());
            println!("{:?}", m);
        }

        board.bitboard().print();
    }
    println!("{:?}", board.gamestate());
    */
}
