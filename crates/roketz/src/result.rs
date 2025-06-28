pub fn handle_result<V>(result: anyhow::Result<V>) -> V {
    result.unwrap_or_else(|e| {
        eprintln!("An error occurred: {e:?}");
        std::process::exit(1);
    })
}

pub fn handle_result_closure<F, V>(f: F) -> V
where
    F: FnOnce() -> anyhow::Result<V>,
{
    handle_result(f())
}

pub async fn handle_result_async_closure<F, V>(f: F) -> V
where
    F: std::future::Future<Output = anyhow::Result<V>>,
{
    handle_result(f.await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_result_success() {
        let result: anyhow::Result<i32> = Ok(42);
        let value = handle_result(result);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_handle_result_closure_success() {
        let value = handle_result_closure(|| Ok::<i32, anyhow::Error>(42));
        assert_eq!(value, 42);
    }
}
