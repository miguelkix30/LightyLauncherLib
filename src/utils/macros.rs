#[macro_export]
macro_rules! join_and_mkdir {
    ($path:expr, $join:expr) => {{
        let path = $path.join($join);
        $crate::mkdir!(&path);
        path
    }};
}

#[macro_export]
macro_rules! join_and_mkdir_vec {
    ($path:expr, $joins:expr) => {{
        let mut path = $path.to_path_buf();
        for join in $joins {
            path = path.join(join);
            $crate::mkdir!(&path);
        }
        path
    }};
}

#[macro_export]
macro_rules! mkdir {
    ($path:expr) => {
        if !$path.exists() {
            if let Err(e) = tokio::fs::create_dir_all(&$path).await {
                error!("Failed to create directory {:?}: {}", $path, e);
            }
        }
    };
}

// Blocking version for sync contexts (if needed)
#[macro_export]
macro_rules! mkdir_blocking {
    ($path:expr) => {
        if !$path.exists() {
            let path = $path.to_path_buf();
            tokio::task::spawn_blocking(move || {
                std::fs::create_dir_all(&path)
            }).await.ok();
        }
    };
}


#[macro_export]
macro_rules! time_it {
    ($label:expr, $expr:expr) => {{
        let start = std::time::Instant::now();
        let result = $expr;
        let elapsed = start.elapsed();
        tracing::debug!(label = $label, elapsed = ?elapsed, "Operation completed");
        result
    }};
}
