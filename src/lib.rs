#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

extern crate clap;

extern crate hyper;

extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_ecs;
extern crate rusoto_ssm;

pub mod command;
pub mod config;