use clap::{arg, App, AppSettings, crate_version, crate_authors, crate_description};

pub enum UseCase {
    Use(String),
}

pub struct Command<'a>
{
    app: App<'a>,
}

impl<'a> Command<'a> {
    pub fn new() -> Command<'a> {
        let app = App::new("envm")
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                App::new("use")
                    .about("Use environment")
                    .arg(arg!(<ENV> "The environment to target"))
            );

        Command { app }
    }

    pub fn run(self) -> UseCase {
        let matches = self.app.get_matches();
        match matches.subcommand() {
            Some(("use", sub_matches)) => {
                let env = sub_matches.value_of("ENV").expect("required");
                return UseCase::Use(String::from(env));
            },
            _ => unreachable!(),
        };
    }
}