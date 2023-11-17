mod cli;
mod db;
mod manager;

use manager::random;

fn main() {
    println!("{0:?}", random::int_randGen());
}
