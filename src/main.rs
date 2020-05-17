mod ls;

use std::path::PathBuf;
use ls::Ls;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
	#[structopt(default_value = ".", parse(from_os_str))]
	path: PathBuf,
}

fn main() {
	let opt = Opt::from_args();
	let list = Ls{};

	match list.run(&opt.path) {
		Ok(_) => println!("Successfully read from directory"),
		Err(e) => println!("Failed with: {}", e),
	}
}