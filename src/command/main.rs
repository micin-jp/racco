use std::error;
use clap::{App, AppSettings, Arg, SubCommand};

use config;
use output;

use super::{DeployCommand, ParamsDeleteCommand, ParamsExecCommand, ParamsGetCommand,
            ParamsListCommand, ParamsPutCommand, RunTaskCommand, ScheduleTaskDeleteCommand,
            ScheduleTaskPutCommand};

pub struct MainCommand {}

impl MainCommand {
    pub fn run() -> Result<(), Box<error::Error>> {

        let matches = App::new("Racco")
            .version("0.1.0")
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
            .subcommand(
                SubCommand::with_name("deploy")
                    .about("Deploys ECS service")
                    .arg(
                        Arg::with_name("NAME")
                            .help("Name of the entry in config")
                            .index(1),
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
                                    .required(true)
                                    .index(1),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("delete")
                            .about("Deletes a scheduled task")
                            .arg(
                                Arg::with_name("NAME")
                                    .help("Name of the entry in config")
                                    .required(true)
                                    .index(1),
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

        info!("start racco");

        let arg_conf = matches.value_of("CONFIG").unwrap_or("racco.yml");
        info!("config file: {}", arg_conf);

        match config::command::Config::from_file(arg_conf) {
            Err(error) => error!("invalid config: {}", error),
            Ok(config) => {

                // deploy
                if let Some(sub_matches) = matches.subcommand_matches("deploy") {

                    info!("start deploy");

                    let cmd = DeployCommand::from_args(&config, sub_matches);
                    match cmd.run() {
                        Ok(_) => {}
                        Err(error) => error!("deploy failed: {}", error),
                    }

                    info!("end deploy");
                }

                // run-task
                if let Some(sub_matches) = matches.subcommand_matches("run-task") {

                    info!("start run-task");

                    let cmd = RunTaskCommand::from_args(&config, sub_matches);
                    match cmd.run() {
                        Ok(_) => {}
                        Err(error) => {
                            output::PrintLine::error(&format!("Failed: {}", error));
                        }
                    }

                    info!("end run-task");
                }

                // schedule-task
                if let Some(sub0_matches) = matches.subcommand_matches("schedule-task") {
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("put") {
                        info!("start schedule-task put");

                        let cmd = ScheduleTaskPutCommand::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {}
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                            }
                        }

                        info!("end schdule-task put");
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("delete") {
                        info!("start schedule-task delete");

                        let cmd = ScheduleTaskDeleteCommand::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {}
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                            }
                        }

                        info!("end schedule-task delete");
                    }
                }

                // params
                if let Some(sub0_matches) = matches.subcommand_matches("params") {
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("get") {
                        info!("start params get");

                        let cmd = ParamsGetCommand::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {}
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                            }
                        }

                        info!("end params get");
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("list") {
                        info!("start params list");

                        let cmd = ParamsListCommand::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {}
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                            }
                        }

                        info!("end params list");
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("put") {
                        info!("start params put");

                        let cmd = ParamsPutCommand::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {}
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                            }
                        }

                        info!("end params put");
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("delete") {
                        info!("start params delete");

                        let cmd = ParamsDeleteCommand::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {}
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                            }
                        }

                        info!("end params delete");
                    }
                    if let Some(sub1_matches) = sub0_matches.subcommand_matches("exec") {
                        info!("start params exec");

                        let cmd = ParamsExecCommand::from_args(&config, sub1_matches);
                        match cmd.run() {
                            Ok(_) => {}
                            Err(error) => {
                                output::PrintLine::error(&format!("Failed: {}", error));
                            }
                        }

                        info!("end params exec");
                    }
                }

            }
        };

        info!("end racco");

        Ok(())
    }
}
