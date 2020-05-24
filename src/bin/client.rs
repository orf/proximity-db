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

fn main() -> anyhow::Result<()> {
    // let opt = Opt::from_args();
    // println!("{:?}", opt);

    let thing = Sky::default();
    thing.add(vec![1.0])?;
    Ok(())
}
