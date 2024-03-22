pub mod api;
pub mod category;

use api::get_data;
use category::Category;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "PokeAPI CLI", about = "Interact with PokeAPI via CLI")]
struct Args {
    /// Category can be pokemon or item.
    #[structopt(short, long, default_value = "pokemon")]
    category: Category,

    /// Key can be either the name or id of subject in category.
    #[structopt(short, long)]
    key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::from_args();

    let value = get_data(args.category.to_string(), args.key).await;

    match value {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error: {}", e),
    }
}
