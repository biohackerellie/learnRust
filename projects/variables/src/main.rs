const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missles, ready) = (STARTING_MISSLES, READY_AMOUNT);

    println!("Firing {} of my {} missles...", ready, missles);

    missles = missles - ready;

    println!("{} missles left", missles)
}
