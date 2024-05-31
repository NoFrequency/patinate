use patination::database::Database;
use patination::miner::Miner;

fn main() {
    let sample_synthetic_data: &[String] = &["abcdefghijklmnopqrstuvxyz".to_string()];
    let database = Database::from(sample_synthetic_data);
    let miner = Miner::with_database(database);

    println!("Hello, patination {}!", miner.mdl_size());
}
