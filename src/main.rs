use structopt::StructOpt;

mod fetch;
mod parser;
mod codes;


#[tokio::main]
async fn main () {
    let args = parser::Opt::from_args();

    if args.debug {
        println!("code: {}", &args.currency_code);
    }

    let rates_resp = fetch::rates(&args).await.unwrap();

    println!("{:#?}", rates_resp.rates);
}

