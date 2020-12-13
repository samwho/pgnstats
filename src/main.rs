use std::{fs::File, io, path::PathBuf, time::Duration};
use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus};
use structopt::StructOpt;
use pbr::{ProgressBar, Units};
use progress_streams::ProgressReader;

#[derive(StructOpt, Debug)]
struct Opt {
    path: PathBuf,
}

struct MoveCounter {
    moves: u128,
}

impl MoveCounter {
    fn new() -> MoveCounter {
        MoveCounter { moves: 0 }
    }
}

impl Visitor for MoveCounter {
    type Result = u128;

    fn begin_game(&mut self) {
    }

    fn san(&mut self, _san_plus: SanPlus) {
        self.moves += 1;
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn end_game(&mut self) -> Self::Result {
        self.moves
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let mut file = File::open(opt.path)?;
    let filesize = file.metadata()?.len();
    let mut pb = ProgressBar::new(filesize);
    pb.set_max_refresh_rate(Some(Duration::from_millis(333)));
    pb.set_units(Units::Bytes);
    let pr = ProgressReader::new(&mut file, |bytes| {
        pb.add(bytes as _);
    });
    let mut reader = BufferedReader::new(pr);

    let mut counter = MoveCounter::new();
    reader.read_all(&mut counter)?;

    pb.finish();

    println!("moves: {}", counter.moves);

    Ok(())
}
