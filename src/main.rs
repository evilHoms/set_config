use std::process;

use set_config::{ get_config_names, set_config, get_env_name, print_info };

fn main() {
    let config_names = get_config_names();
    let config_names = config_names.unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    if let Some(env) = get_env_name() {
        if !config_names.contains(&env) {
            eprintln!("Wrong config name!");
            process::exit(1);
        }

        if let Err(e) = set_config(&env) {
            eprintln!("{}", e);
            process::exit(1);
        }
    } else {
        print_info(config_names);
    }
}
