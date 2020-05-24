use embedding_db::sky::Sky;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "v", long = "velocity", default_value = "42")]
    speed: f64,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out` is set to `file`
    #[structopt(name = "FILE", required_if("out_type", "file"))]
    file_name: String,
}

use crossbeam_channel::{bounded, unbounded};
use embedding_db::help::Container;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

fn main() -> anyhow::Result<()> {
    // let opt = Opt::from_args();
    // println!("{:?}", opt);

    let mut thing = Arc::new(RwLock::new(Sky::default()));
    {
        let mut mut_thing = thing.write().unwrap();
        for _ in 0..100 {
            mut_thing.add("some-name".to_string(), vec![1.0, 2.0, 3.0])?;
        }
    }
    let (sender, receiver) = bounded(1);

    let another_thing = thing.clone();
    spawn(move || {
        another_thing.read().unwrap().query(
            "some-name".to_string(),
            10.0,
            vec![1.0, 2.0, 3.0],
            sender,
        )
    });

    let mut thing = receiver.iter();
    thing.next();
    drop(receiver);
    Ok(())
}
