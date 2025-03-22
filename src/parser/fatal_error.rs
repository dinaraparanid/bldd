pub trait FatalError<T, E> {
    fn get_or_exit<F : FnOnce(E) -> String>(self, message: F) -> T;
}

impl <T, E> FatalError<T, E> for Result<T, E> {
    fn get_or_exit<F : FnOnce(E) -> String>(self, message: F) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                println!("{}", message(e));
                std::process::exit(-1);
            },
        }
    }
}
