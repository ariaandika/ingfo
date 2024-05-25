pub mod cli;

use ingfo::disk::list_partitions;


fn main() {
    println!("Partitions: {:#?}", list_partitions());
    cli::main();
}

