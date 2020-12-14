use pbr::{ProgressBar, Units};
use pgn_reader::{BufferedReader, San, SanPlus, Skip, Square, Visitor};
use progress_streams::ProgressReader;
use std::{collections::BTreeMap, fs::File, io, path::PathBuf, time::Duration};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    path: PathBuf,
}

struct CaptureCounter {
    captures: BTreeMap<Square, usize>,
}

impl CaptureCounter {
    fn new() -> Self {
        Self {
            captures: BTreeMap::new(),
        }
    }
}

impl Visitor for CaptureCounter {
    type Result = ();

    fn begin_game(&mut self) {}

    fn san(&mut self, san: SanPlus) {
        if let San::Normal { capture, to, .. } = san.san {
            if capture {
                let count = self.captures.entry(to).or_insert(0);
                *count += 1;
            }
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn end_game(&mut self) -> Self::Result {
        ()
    }

    fn end_headers(&mut self) -> Skip {
        Skip(false)
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

    let mut visitor = CaptureCounter::new();
    reader.read_all(&mut visitor)?;

    pb.finish();

    let total = visitor
        .captures
        .iter()
        .fold(0, |total, (_, count)| total + count);

    let mut sorted: Vec<(&Square, &usize)> = visitor.captures.iter().collect();
    sorted.sort_by(|a, b| a.1.cmp(b.1));

    for (square, count) in sorted {
        println!(
            "{} - {:.2}%",
            square,
            (*count as f64 / total as f64) * 100.0
        );
    }

    Ok(())
}
