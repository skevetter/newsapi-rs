use std::future::Future;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub enum RetryStrategy {
    #[default]
    None,
    Constant(Duration),
    Linear(Duration),
    Exponential(Duration),
}

pub async fn retry<F, T, E, Fut>(
    strategy: RetryStrategy,
    max_retries: usize,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    match strategy {
        RetryStrategy::None => operation().await,
        _ => {
            let mut attempt = 0;

            loop {
                match operation().await {
                    Ok(result) => return Ok(result),
                    Err(_e) if attempt < max_retries => {
                        let delay = match strategy {
                            RetryStrategy::None => Duration::from_secs(0),
                            RetryStrategy::Constant(d) => d,
                            RetryStrategy::Linear(d) => {
                                Duration::from_millis((d.as_millis() as u64) * (attempt + 1) as u64)
                            },
                            RetryStrategy::Exponential(d) => {
                                Duration::from_millis((d.as_millis() as u64) * (2_u64.pow(attempt as u32)))
                            },
                        };
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

#[cfg(feature = "blocking")]
pub fn retry_blocking<F, T, E>(
    strategy: RetryStrategy,
    max_retries: usize,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    match strategy {
        RetryStrategy::None => operation(),
        _ => {
            let mut attempt = 0;

            loop {
                match operation() {
                    Ok(result) => return Ok(result),
                    Err(_e) if attempt < max_retries => {
                        let delay = match strategy {
                            RetryStrategy::None => Duration::from_secs(0),
                            RetryStrategy::Constant(d) => d,
                            RetryStrategy::Linear(d) => {
                                Duration::from_millis((d.as_millis() as u64) * (attempt + 1) as u64)
                            },
                            RetryStrategy::Exponential(d) => {
                                Duration::from_millis((d.as_millis() as u64) * (2_u64.pow(attempt as u32)))
                            },
                        };
                        std::thread::sleep(delay);
                        attempt += 1;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_none() {
        let counter = std::cell::Cell::new(0);
        let result = retry(RetryStrategy::None, 3, || async {
            counter.set(counter.get() + 1);
            Ok::<_, ()>(counter.get())
        }).await;

        assert_eq!(result.unwrap(), 1);
        assert_eq!(counter.get(), 1);
    }

    #[tokio::test]
    async fn test_retry_constant() {
        let counter = std::cell::Cell::new(0);
        let result = retry(RetryStrategy::Constant(Duration::from_millis(1)), 3, || async {
            counter.set(counter.get() + 1);
            if counter.get() < 3 {
                Err("error")
            } else {
                Ok(counter.get())
            }
        }).await;

        assert_eq!(result.unwrap(), 3);
        assert_eq!(counter.get(), 3);
    }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let counter = std::cell::Cell::new(0);
        let result = retry(RetryStrategy::Constant(Duration::from_millis(1)), 2, || async {
            counter.set(counter.get() + 1);
            Err::<i32, _>("always fails")
        }).await;

        assert!(result.is_err());
        assert_eq!(counter.get(), 3); // Initial attempt + 2 retries
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn test_retry_blocking_function() {
        let mut counter = 0;
        let result = retry_blocking(RetryStrategy::Constant(Duration::from_millis(1)), 2, || {
            counter += 1;
            if counter < 2 {
                Err("error")
            } else {
                Ok(counter)
            }
        });

        assert_eq!(result.unwrap(), 2);
        assert_eq!(counter, 2);
    }
}
