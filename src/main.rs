use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("myapp")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add todo.")
                .arg(Arg::with_name("text").index(1).required(true)),
        )
        .subcommand(SubCommand::with_name("show"))
        .get_matches();
    if let Some(ref matches) = matches.subcommand_matches("add") {
        println!("Add item: {}", matches.value_of("text").unwrap());
    }
    if let Some(..) = matches.subcommand_matches("show") {
        println!("Show all todos,");
    }
}
