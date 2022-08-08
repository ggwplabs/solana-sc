use clap::{App, AppSettings, Arg};

pub fn get_clap_app<'a, 'b>(name: &'a str, desc: &'a str, version: &'a str) -> App<'a, 'b> {
    App::new(name)
        .about(desc)
        .version(version)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Path to configuration file."),
        )
}
