mod ast;
mod builder;
mod primitive_builders;

use std::path::Path;

use log::error;

fn main() {
    env_logger::init();

    //  Get file from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: logix_lang <file>");
        return;
    }

    let file = &args[1];
    let path = Path::new(file).canonicalize().unwrap();
    let path_str = path.to_str().unwrap();

    let (comp, id_map) = match builder::build_from_file(path_str) {
        Ok(comp) => comp,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };
}
