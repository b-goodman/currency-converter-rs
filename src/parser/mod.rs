use structopt::StructOpt;
use crate::codes;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(short, long)]
    /// Print status and output during evaluation.
    pub debug: bool,
    /// Three char. currency code. E.g., USD, GBP, EUR.
    pub currency_code: codes::Currency,
}

