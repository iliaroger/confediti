use clap::{Arg, ArgMatches, Command, Subcommand};

fn main() {
    let matches = Command::new("confucius")
        .version("0.1")
        .author("Ilya Roger")
        .about("Create, update and migrate your configs hasle free.")
        .subcommand(Command::new("get"))
        .about("gets the config files and installs them into your machine")
        .arg(Arg::new("url"))
        .short_flag('u')
        .long_flag("url")
        .subcommand_value_name("URL");
}
