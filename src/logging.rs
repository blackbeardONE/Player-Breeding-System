use flexi_logger::{Logger, Duplicate, Criterion, Naming, Cleanup, FileSpec};

pub fn init_logging() {
    Logger::try_with_env_or_str("info")
        .unwrap()
        .duplicate_to_stderr(Duplicate::Info)
        .log_to_file(FileSpec::default())
        .rotate(
            Criterion::Size(10_000_000), // 10 MB
            Naming::Numbers,
            Cleanup::KeepLogFiles(7),
        )
        .start()
        .unwrap();
}
