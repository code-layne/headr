use headr::get_args;

fn main() {
    if let Err(e) = headr::run(get_args().unwrap()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
