use log::LevelFilter;

pub fn init_logging() {
    // Make this idempotent – calling it multiple times in tests / main is fine.
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        // Default filter can be overridden via `RUST_LOG`
        let mut builder =
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));

        // Always log to stdout
        builder.target(env_logger::Target::Stdout);

        // For tests: use `is_test(true)` and DEBUG level by default
        #[cfg(test)]
        {
            builder.is_test(true);
            builder.filter_level(LevelFilter::Debug);
        }

        // For normal binary runs: INFO level by default
        #[cfg(not(test))]
        {
            builder.filter_level(LevelFilter::Info);
        }

        let _ = builder.try_init();
    });
}
