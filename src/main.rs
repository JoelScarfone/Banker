mod account;
mod amount;
mod bank;
mod cli;
mod error;
mod transaction;

use clap::Parser;

use csv::ReaderBuilder;

use crate::{bank::Bank, error::Error};

fn main() {
    let args = cli::Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}

fn run(args: cli::Args) -> Result<(), Error> {
    // Create our bank
    let mut bank = Bank::new();

    // load input from csv
    let mut reader = match ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(true)
        .flexible(true)
        .from_path(args.file())
    {
        Ok(reader) => reader,
        Err(e) => {
            return Err(Error::with_cause(
                format!("Failed to create reader for: {:?}", args.file()),
                e,
            ));
        }
    };

    // stream from the csv, processing each transaction one at a time
    for result in reader.deserialize() {
        let transaction: crate::transaction::Transaction = result?;
        bank.process_transaction(transaction);
    }

    // stream to stdout
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(std::io::stdout());
    writer.serialize(("client", "available", "held", "total", "locked"))?;
    for row in bank.accounts_iter() {
        writer.serialize(row)?;
    }

    Ok(())
}
