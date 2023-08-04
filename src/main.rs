use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
       println!("{}", args.input);
   }
}
