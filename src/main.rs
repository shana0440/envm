use envm;

fn main() {
    if let Err(err) = envm::run() {
        eprintln!("{}", err);
    }
}
