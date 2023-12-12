use log::LevelFilter;

pub fn init(verbose_level: u8) {
    let level = match verbose_level {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::builder()
        .filter_level(level)
        .format_timestamp(None)
        .init();
}
