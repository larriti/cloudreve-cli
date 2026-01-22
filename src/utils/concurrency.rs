use futures::stream::{self, StreamExt};
use std::future::Future;

/// 带并发控制的异步任务执行
pub async fn execute_with_concurrency<F, T, E>(
    tasks: Vec<(String, F)>,
    concurrency: usize,
) -> Vec<(String, Result<T, E>)>
where
    F: Future<Output = Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    if concurrency == 0 || concurrency >= tasks.len() {
        // 无并发限制或任务数少于并发限制，全部并发执行
        let handles: Vec<_> = tasks
            .into_iter()
            .map(|(name, task)| {
                tokio::spawn(async move {
                    let result = task.await;
                    (name, result)
                })
            })
            .collect();

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }
        results
    } else {
        // 使用 buffer_unordered 控制并发数
        let mut results = Vec::new();

        let mut stream = stream::iter(tasks)
            .map(|(name, task)| async move { (name, task.await) })
            .buffer_unordered(concurrency);

        while let Some(result) = stream.next().await {
            results.push(result);
        }
        results
    }
}
