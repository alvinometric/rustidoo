use pico_args;
fn main() {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-n", "--new"]) {
        println!("{}", "new");
        std::process::exit(0);
    }
    println!("Hello, world!");
}
