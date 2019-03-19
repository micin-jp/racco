use std::collections::BTreeMap;
use std::env;
use std::error;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use config;
use output;

use super::error::CommandError;

use super::configtest;
use super::params;
use super::run_task;
use super::schedule_task;
use super::service;

pub struct MainCommand {}

impl MainCommand {
    fn validate_args_template_variables(tag_str: String) -> Result<(), String> {
        let pair: Vec<&str> = tag_str.split("=").collect();
        if pair.len() == 2 {
            Ok(())
        } else {
            Err(String::from(
                "The variable format should contain variable name and variable value, and those are seperated with `=` character",
            ))
        }
    }
    fn parse_args_template_variables(args: &ArgMatches) -> Option<BTreeMap<String, String>> {
        args.values_of("CONFIG_TEMPLATE_VARIABLES").map(|vars_str| {
            let mut data = BTreeMap::new();
            for var_str in vars_str {
                let pair: Vec<&str> = var_str.split("=").collect();
                data.insert(pair[0].to_owned(), pair[1].to_owned());
            }

            data
        })
    }

    fn config_file(args: &ArgMatches) -> String {
        if let Some(config_file) = args.value_of("CONFIG") {
            return config_file.to_owned();
        }
        if let Ok(config_file) = env::var("RACCO_CONFIG_PATH") {
            return config_file;
        }

        String::from("racco.yml")
    }

    pub fn run() -> Result<(), Box<error::Error>> {
        let matches = App::new("Racco")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Daichi Sakai. <daisaru11@gmail.com>")
            .about("Deployment toolkit for AWS ECS")
            .arg(
                Arg::with_name("CONFIG")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Specifies configuration file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("CONFIG_TEMPLATE_VARIABLE_FILE")
                    .short("t")
                    .long("config-template-var-file")
                    .value_name("FILENAME")
                    .help("A File defines variables rendered in config template")
                    .takes_value(true)
                    .multiple(true),
            )
            .arg(
                Arg::with_name("CONFIG_TEMPLATE_VARIABLES")
                    .short("a")
                    .long("config-template-vars")
                    .value_name("NAME=VALUE")
                    .help("Variables to be rendered in config template.")
                    .takes_value(true)
                    .multiple(true)
                    .validator(MainCommand::validate_args_template_variables),
            )
            .subcommand(SubCommand::with_name("config").about("Display loaded config"))
            .subcommand(
                SubCommand::with_name("service")
                    .about("Manages ECS services")
                    .subcommand(
                        SubCommand::with_name("deploy")
                            .about("Deploys ECS service")
                            .arg(
                                Arg::with_name("NAME")
                                    .help("Name of the entry in config")
                                    .required_unless("ALL")
                                    .index(1),
                            )
                            .arg(
                                Arg::with_name("ALL")
                                    .help("Deploy all services")
                                    .long("all"),
                            )
                            .arg(
                                Arg::with_name("NO_WAIT")
                                    .help("Do not wait until new tasks to be running")
                                    .long("no-wait"),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("stop")
                            .about("Stops ECS service (change desired count to zero)")
                            .arg(
                                Arg::with_name("NAME")
                                    .help("Name of the entry in config")
                                    .required_unless("ALL")
                                    .index(1),
                            )
                            .arg(Arg::with_name("ALL").help("Stop all services").long("all"))
                            .arg(
                                Arg::with_name("NO_WAIT")
                                    .help("Do not wait until new tasks to be stopped")
                                    .long("no-wait"),
                            ),
                    ),
            )
            .subcommand(
                SubCommand::with_name("run-task")
                    .about("Runs single ESC task")
                    .arg(
                        Arg::with_name("NAME")
                            .help("Name of the entry in config")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("NO_WAIT")
                            .help("Do not wait until new tasks to be running")
                            .long("no-wait"),
                    ),
            )
            .subcommand(
                SubCommand::with_name("schedule-task")
                    .about("Deploys ECS task scheduled by Cloudwatch events")
                    .subcommand(
                        SubCommand::with_name("put")
                            .about("Puts a scheduled task")
                            .arg(
                                Arg::with_name("NAME")
                                    .help("Name of the entry in config")
                                    .required_unless("ALL")
                                    .index(1),
                            )
                            .arg(
                                Arg::with_name("ALL")
                                    .help("Put all schedule tasks")
                                    .long("all"),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("delete")
                            .about("Deletes a scheduled task")
                            .arg(
                                Arg::with_name("NAME")
                                    .help("Name of the entry in config")
                                    .required_unless("ALL")
                                    .index(1),
                            )
                            .arg(
                                Arg::with_name("ALL")
                                    .help("Delete all schedule tasks")
                                    .long("all"),
                            ),
                    ),
            )
            .subcommand(
                SubCommand::with_name("params")
                    .about("Manages parameters")
                    .subcommand(
                        SubCommand::with_name("get").about("Gets a parameter").arg(
                            Arg::with_name("NAME")
                                .help("Name of the parameter")
                                .required(true)
                                .index(1),
                        ),
                    )
                    .subcommand(SubCommand::with_name("list").about("Lists parameters"))
                    .subcommand(
                        SubCommand::with_name("put")
                            .about("Puts a parameter")
                            .arg(
                                Arg::with_name("NAME")
                                    .help("Name of the parameter")
                                    .required(true)
                                    .index(1),
                            )
                            .arg(
                                Arg::with_name("VALUE")
                                    .help("Value of the parameter")
                                    .required(true)
                                    .index(2),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("delete")
                            .about("Deletes a parameter")
                            .arg(
                                Arg::with_name("NAME")
                                    .help("Name of the parameter")
                                    .required(true)
                                    .index(1),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("exec")
                            .setting(AppSettings::AllowLeadingHyphen)
                            .about(
                                "Executes a command with the exported parameters as env variables",
                            )
                            .arg(
                                Arg::with_name("PROGRAM")
                                    .help("Program to be executed")
                                    .required(true)
                                    .index(1),
                            )
                            .arg(
                                Arg::with_name("ARGS")
                                    .help("Arguments passed to the program")
                                    .multiple(true)
                                    .index(2),
                            ),
                    ),
            )
            .get_matches();

        let config_file = MainCommand::config_file(&matches);
        info!("config file: {}", config_file);

        let template_variables = MainCommand::parse_args_template_variables(&matches);

        let template_variable_files = matches.values_of("CONFIG_TEMPLATE_VARIABLE_FILE").map(|v| v.collect());

        match config::command::Config::from_file(
            config_file.as_str(),
            template_variables.as_ref(),
            template_variable_files,
        ) {
            Err(error) => {
                output::PrintLine::error(&format!("Failed loading the configuration: {}", error));
                return Err(error);
            }
            Ok(config) => {
                // service
                if let Some(sub0_matches) = matches.subcommand_matches("service") {
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("deploy") {
                        info!("start service deploy");

                        let cmd = service::deploy::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end service deploy");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!(
                                    "Failed deploying the service: {}",
                                    error
                                ));
                                return Err(error);
                            }
                        }
                    }

                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("stop") {
                        info!("start stopping service");

                        let cmd = service::stop::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end stopping service");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!(
                                    "Failed stopping the service: {}",
                                    error
                                ));
                                return Err(error);
                            }
                        }
                    }
                }

                // config
                if let Some(sub0_matches) = matches.subcommand_matches("config") {
                    info!("start config");

                    let cmd = configtest::Command::from_args(&config, sub0_matches);
                    match cmd.run() {
                        Ok(_) => {
                            info!("end config");
                            return Ok(());
                        }
                        Err(error) => {
                            output::PrintLine::error(&format!("Failed display config: {}", error));
                            return Err(error);
                        }
                    }
                }

                // run-task
                if let Some(sub_matches) = matches.subcommand_matches("run-task") {
                    info!("start run-task");

                    let cmd = run_task::Command::from_args(&config, sub_matches);
                    match cmd.run() {
                        Ok(_) => {
                            info!("end run-task");
                            return Ok(());
                        }
                        Err(error) => {
                            output::PrintLine::error(&format!(
                                "Failed running the task: {}",
                                error
                            ));
                            return Err(error);
                        }
                    }
                }

                // schedule-task
                if let Some(sub0_matches) = matches.subcommand_matches("schedule-task") {
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("put") {
                        info!("start schedule-task put");

                        let cmd = schedule_task::put::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end schdule-task put");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                                return Err(error);
                            }
                        }
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("delete") {
                        info!("start schedule-task delete");

                        let cmd = schedule_task::delete::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end schedule-task delete");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                                return Err(error);
                            }
                        }
                    }
                }

                // params
                if let Some(sub0_matches) = matches.subcommand_matches("params") {
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("get") {
                        info!("start params get");

                        let cmd = params::get::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end params get");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                                return Err(error);
                            }
                        }
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("list") {
                        info!("start params list");

                        let cmd = params::list::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end params list");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                                return Err(error);
                            }
                        }
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("put") {
                        info!("start params put");

                        let cmd = params::put::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end params put");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                                return Err(error);
                            }
                        }
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("delete") {
                        info!("start params delete");

                        let cmd = params::delete::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end params delete");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                                return Err(error);
                            }
                        }
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("exec") {
                        info!("start params exec");

                        let cmd = params::exec::Command::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {
                                info!("end params exec");
                                return Ok(());
                            }
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                                return Err(error);
                            }
                        }
                    }
                }
            }
        };

        Err(Box::new(CommandError::CommandNotFound))
    }
}
