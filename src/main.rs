use std::process;


fn main() {
    use action_validator::CliConfig;

    let res = CliConfig::parse_itr(std::env::args_os());
    match res {
        Ok(config) => {
            let response = action_validator::cli::run(&config);
            if let Some(err) = response.errors {
                eprint!("{err}\n");
                process::exit(response.exit_code);
            }
        },
        Err(e) => {
            print!("{e}");
            process::exit(0);
        },
    };
}
