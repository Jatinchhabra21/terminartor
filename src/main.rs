use terminartor::{cli, parse_args};

fn main() {
    let command = cli().get_matches();

    parse_args(command)
}
