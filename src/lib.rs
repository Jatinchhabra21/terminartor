use ascii::ascii::img_to_ascii;
use clap::Arg;
use clap::ArgMatches;
use clap::Command;

pub mod ascii;

pub fn cli() -> Command {
    Command::new("terminartor")
        .about("converts images to terminal art using ASCII characters")
        .arg_required_else_help(true)
        .arg(Arg::new("path").long("path").short('p').required(true).help("absolute path of the image"))
        .arg(Arg::new("scale")
            .short('s')
            .long("scale")
            .required(true)
            .help("scales the image")
            .long_help("scales the image, example - scale of 2 would mean every 2 pixels will be represented by 1 character, scale of 4 would be 4 pixels for 1 character")
        )
}

pub fn parse_args(cmd_match: ArgMatches) {
    let path = cmd_match.get_one::<String>("path").unwrap();
    let mut scale = cmd_match
        .get_one::<String>("scale")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    if scale > 64 {
        scale = 64;
    } else if scale < 1 {
        scale = 1;
    }

    img_to_ascii(path, scale)
}
