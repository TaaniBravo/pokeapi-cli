pub mod api;

use api::get_data;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "PokeAPI CLI", about = "Interact with PokeAPI via CLI")]
struct Args {
    /// Category can be pokemon or item.
    #[structopt(short, long)]
    category: String,

    /// Key can be either the name or id of subject in category.
    #[structopt(short, long)]
    key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::from_args();

    let value = get_data(args.category, args.key).await;

    match value {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }
}
