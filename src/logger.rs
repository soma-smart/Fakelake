use log::LevelFilter;

fn get_level(verbose_level: u8) -> LevelFilter {
    match verbose_level {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}

pub fn init(verbose_level: u8) {
    let level = get_level(verbose_level);

    let _ = env_logger::builder()
        .filter_level(level)
        .format_timestamp(None)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::LevelFilter;

    #[test]
    fn given_0_should_return_level_info() {
        assert_eq!(get_level(0), LevelFilter::Info);
    }

    #[test]
    fn given_1_should_return_level_debug() {
        assert_eq!(get_level(1), LevelFilter::Debug);
    }

    #[test]
    fn given_other_should_return_level_trace() {
        let values = [2, 30];
        for value in values {
            assert_eq!(get_level(value), LevelFilter::Trace);
        }
    }

    #[test]
    fn given_info_should_compile() {
        init(0);
    }

    #[test]
    fn given_debug_should_compile() {
        init(1);
    }

    #[test]
    fn given_trace_should_compile() {
        init(2);
    }
}
