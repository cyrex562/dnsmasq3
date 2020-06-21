use log::{debug};

pub fn init_logger() -> Result<(), fern::InitError> {
    // let formatter = syslog::Formatter3164 {
    //     facility: syslog::Facility::LOG_USER,
    //     hostname: None,
    //     process: "prism_tank".to_owned(),
    //     pid: 0,
    // };
    fern::Dispatch::new()
    .chain(
        fern::Dispatch::new()
            .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}:{}:{}:{}",
                chrono::Local::now().format("%Y-%m-%d-%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout())
    )
    // .chain(
    //     fern::Dispatch::new()
    //         .level(log::LevelFilter::Info)
    //         .chain(syslog::unix(syslog::Facility::LOG_USER)?)
    // )    
    .apply()?;

    Ok(())
}