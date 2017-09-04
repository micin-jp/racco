use std::error;
use config;

use clap::{Arg, App, SubCommand, AppSettings};

use super::{DeployCommand, RunTaskCommand, ScheduleTaskPutCommand, ScheduleTaskDeleteCommand, ParamsExecCommand, ParamsGetCommand, ParamsPutCommand, ParamsDeleteCommand};

pub struct MainCommand {
}

impl MainCommand {

  pub fn run() -> Result<(), Box<error::Error>> {

    let matches = App::new("Racco")
                    .version("0.0.1")
                    .author("Daichi Sakai. <daisaru11@gmail.com>")
                    .about("Utilities to deploy ECS")
                    .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Configuration file")
                            .takes_value(true))
                    .subcommand(SubCommand::with_name("deploy")
                        .about("deploy ECS service")
                        .arg(Arg::with_name("NAME")
                            .help("name")
                            .index(1))
                    )
                    .subcommand(SubCommand::with_name("run-task")
                        .about("run ECS single task")
                        .arg(Arg::with_name("NAME")
                            .help("name")
                            .required(true)
                            .index(1))
                    )
                    .subcommand(SubCommand::with_name("schedule-task")
                        .about("deploy ECS task scheduled by Cloudwatch events")
                        .subcommand(SubCommand::with_name("put")
                            .about("put scheduled task")
                            .arg(Arg::with_name("NAME")
                                .help("name")
                                .required(true)
                                .index(1))
                        )
                        .subcommand(SubCommand::with_name("delete")
                            .about("delete scheduled task")
                            .arg(Arg::with_name("NAME")
                                .help("name")
                                .required(true)
                                .index(1))
                        )
                    )
                    .subcommand(SubCommand::with_name("params")
                        .about("manages parameters")
                        .subcommand(SubCommand::with_name("get")
                            .about("get parameters")
                            .arg(Arg::with_name("NAME")
                               .help("Parameter name")
                               .required(true)
                               .index(1))
                        )
                        .subcommand(SubCommand::with_name("put")
                            .about("put parameters")
                            .arg(Arg::with_name("NAME")
                               .help("Parameter name")
                               .required(true)
                               .index(1))
                            .arg(Arg::with_name("VALUE")
                               .help("Parameter value")
                               .required(true)
                               .index(2))
                        )
                        .subcommand(SubCommand::with_name("delete")
                            .about("delete parameters")
                            .arg(Arg::with_name("NAME")
                               .help("Parameter name")
                               .required(true)
                               .index(1))
                        )
                        .subcommand(SubCommand::with_name("exec")
                            .setting(AppSettings::AllowLeadingHyphen)
                            .about("executes a command with exported parameters as env variables")
                            .arg(Arg::with_name("PROGRAM")
                                .help("Program")
                                .required(true)
                                .index(1))
                            .arg(Arg::with_name("ARGS")
                                .help("Arguments")
                                .multiple(true)
                                .index(2)
                                )
                        )
                    )
                    .get_matches()
                    ;

    info!("start racco");

    let arg_conf = matches.value_of("config").unwrap_or("racco.yml");
    info!("config file: {}", arg_conf);

    match config::command::Config::from_file(arg_conf) {
        Err(error) => {
            error!("invalid config: {}", error)
        }
        Ok(config) => {
         
            // deploy
            if let Some(sub_matches) = matches.subcommand_matches("deploy") {

                info!("start deploy");

                let cmd = DeployCommand::from_args(&config, sub_matches);
                match cmd.run() {
                    Ok(_) => {
                    },
                    Err(error) => {
                        error!("deploy failed: {}", error)
                    }
                }

                info!("end deploy");
            }

            // run-task
            if let Some(sub_matches) = matches.subcommand_matches("run-task") {

                info!("start run-task");

                let cmd = RunTaskCommand::from_args(&config, sub_matches);
                match cmd.run() {
                    Ok(_) => {
                    },
                    Err(error) => {
                        error!("run-task failed: {}", error)
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
                        Ok(_) => {
                        },
                        Err(error) => {
                            error!("schedule-task put failed: {}", error)
                        }
                    }

                    info!("end schdule-task put");
                }
                if let Some(sub1_matches) = sub0_matches.subcommand_matches("delete") {
                    info!("start schedule-task delete");

                    let cmd = ScheduleTaskDeleteCommand::from_args(&config, sub1_matches);
                    match cmd.run() {
                        Ok(_) => {
                        },
                        Err(error) => {
                            error!("schedule-task delete failed: {}", error)
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
                        Ok(_) => {
                        },
                        Err(error) => {
                            error!("params get failed: {}", error)
                        }
                    }

                    info!("end params get");
                }
                if let Some(sub1_matches) = sub0_matches.subcommand_matches("put") {
                    info!("start params put");

                    let cmd = ParamsPutCommand::from_args(&config, sub1_matches);
                    match cmd.run() {
                        Ok(_) => {
                        },
                        Err(error) => {
                            error!("params put failed: {}", error)
                        }
                    }

                    info!("end params put");
                }
                if let Some(sub1_matches) = sub0_matches.subcommand_matches("delete") {
                    info!("start params delete");

                    let cmd = ParamsDeleteCommand::from_args(&config, sub1_matches);
                    match cmd.run() {
                        Ok(_) => {
                        },
                        Err(error) => {
                            error!("params delete failed: {}", error)
                        }
                    }

                    info!("end params delete");
                }
                if let Some(sub1_matches) = sub0_matches.subcommand_matches("exec") {
                    info!("start params exec");

                    let cmd = ParamsExecCommand::from_args(&config, sub1_matches);
                    match cmd.run() {
                        Ok(_) => {
                        },
                        Err(error) => {
                            error!("params exec failed: {}", error)
                        }
                    }

                    info!("end params exec");
                }
            }

        },
    };

    info!("end racco");
    
    Ok(())
  }
}
