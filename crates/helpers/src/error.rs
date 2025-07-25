use tracing::error;

pub trait HandleError<T> {
    fn handle(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Debug> HandleError<T> for Result<T, E> {
    fn handle(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                if tracing::enabled!(tracing::Level::ERROR) {
                    error!("{msg}: {e:?}");
                } else {
                    eprintln!("{msg}: {e:?}");
                }
                std::process::exit(1);
            }
        }
    }
}
