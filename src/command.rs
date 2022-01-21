use clap::{arg, crate_authors, crate_description, crate_version, App, AppSettings};

pub enum UseCase {
    UseEnvironment(String),
    NewEnvironment(String),
    ListEnvironments,
    InitConfiguration,
}

pub struct Command<'a> {
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
                    .arg(arg!(<ENV> "The environment to target")),
            )
            .subcommand(App::new("init").about("Create envm repository"))
            .subcommand(
                App::new("new")
                    .about("Create new environment base on template")
                    .arg(arg!(<ENV> "The environment to target")),
            )
            .subcommand(App::new("ls").about("List all available environments except local"));

        Command { app }
    }

    pub fn run(self) -> UseCase {
        let matches = self.app.get_matches();
        match matches.subcommand() {
            Some(("use", sub_matches)) => {
                let env = sub_matches.value_of("ENV").expect("required");
                return UseCase::UseEnvironment(String::from(env));
            }
            Some(("init", _)) => {
                return UseCase::InitConfiguration;
            }
            Some(("new", sub_matches)) => {
                let env = sub_matches.value_of("ENV").expect("required");
                return UseCase::NewEnvironment(String::from(env));
            }
            Some(("ls", _)) => {
                return UseCase::ListEnvironments;
            }
            _ => unreachable!(),
        };
    }
}
