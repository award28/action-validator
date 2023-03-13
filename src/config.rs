use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "action-validator",
    about = "A validator for GitHub Action and Workflow YAML files",
    version
)]
pub struct CliConfig {
    /// Be more verbose
    #[arg(short, long)]
    pub verbose: bool,

    /// Input file
    #[arg(name = "path_to_action_yaml")]
    pub src: Vec<PathBuf>,
}

use std::ffi::OsString;

use clap::{CommandFactory, FromArgMatches, Error};

impl CliConfig {
    fn format_error<I: CommandFactory>(err: Error) -> Error {
        let mut cmd = I::command();
        err.format(&mut cmd)
    }

    pub fn parse_itr<I, T>(itr: I) -> Result<Self, String>
        where
            I: IntoIterator<Item = T>,
            T: Into<OsString> + Clone,
    {
        let arg_matches = <Self as CommandFactory>::command()
            .try_get_matches_from_mut(itr);
    
        if let Err(e) = arg_matches {
            return Err(format!("{e}"));
        }
        let mut matches = arg_matches.unwrap();
        let res = <Self as FromArgMatches>::from_arg_matches_mut(&mut matches)
            .map_err(Self::format_error::<Self>);
        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("{e}")),
        }
    }
}

#[derive(Serialize, Copy, Clone, Debug)]
pub enum ActionType {
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "workflow")]
    Workflow,
}

pub struct JsConfig<'a> {
    pub action_type: ActionType,
    pub src: &'a str,
    pub verbose: bool,
}

pub struct RunConfig<'a> {
    pub file_path: Option<&'a str>,
    pub file_name: Option<&'a str>,
    pub action_type: ActionType,
    pub src: &'a str,
    pub verbose: bool,
}

impl<'a> From<&JsConfig<'a>> for RunConfig<'a> {
    fn from(config: &JsConfig<'a>) -> Self {
        RunConfig {
            file_path: None,
            file_name: None,
            action_type: config.action_type,
            src: config.src,
            verbose: config.verbose,
        }
    }
}
