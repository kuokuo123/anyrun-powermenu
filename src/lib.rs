use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::{Deserialize, Serialize};
use std::{fs, process::Command};
use fuzzy_matcher::FuzzyMatcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Engine {
    Logout,
    Suspend,
    Hibernate,
    Reboot,
    Shutdown,
    Custom { name: String, cmd: String, icon: String },
}

impl Engine {
    fn name(&self) -> &str {
        match self {
            Self::Logout => "Logout",
            Self::Suspend => "Suspend",
            Self::Hibernate => "Hibernate",
            Self::Reboot => "Reboot",
            Self::Shutdown => "Shutdown",
            Self::Custom { name, .. } => name,
        }
    }
    fn cmd(&self) -> &str {
        match self {
            Self::Logout => "loginctl terminate-user $USER",
            Self::Suspend => "systemctl suspend",
            Self::Hibernate => "systemctl hibernate",
            Self::Reboot => "systemctl reboot",
            Self::Shutdown => "systemctl poweroff",
            Self::Custom { cmd, .. } => cmd,
        }
    }
    fn icon(&self) -> &str {
        match self {
            Self::Logout => "system-log-out",
            Self::Suspend => "system-suspend",
            Self::Hibernate => "system-hibernate",
            Self::Reboot => "system-restart",
            Self::Shutdown => "system-shutdown",
            Self::Custom { icon, .. } => icon,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    prefix: String,
    engines: Vec<Engine>,
    max_entries: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: "p ".to_string(),
            engines: vec![Engine::Logout, Engine::Reboot, Engine::Shutdown],
            max_entries: 12,
        }
    }
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/powermenu.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Power Menu".into(),
        icon: "cs-power".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, config: &Config) -> RVec<Match> {
    let input = if let Some(input) = input.strip_prefix(&config.prefix) {
        input.trim()
    } else {
        return RVec::new();
    };
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().ignore_case();
    let mut engines = config
        .engines
        .iter()
        .filter_map(|engine| {
            matcher
                .fuzzy_match(&engine.name(), input)
                .map(|score| (engine, score))
        })
        .collect::<Vec<_>>();

    engines.sort_by(|a, b| b.1.cmp(&a.1));

    engines.truncate(config.max_entries);

    engines
        .into_iter()
        .map(|(engine, _)| Match {
            title: engine.name().into(),
            description: ROption::RSome(format!("{}", engine.cmd()).into()),
            use_pango: false,
            icon: ROption::RSome(format!("{}", engine.icon()).into()),
            id: ROption::RNone,
        })
        .collect()
}

#[handler]
fn handler(selection: Match) -> HandleResult {

    let cmd = selection.description.unwrap();

    if let Err(why) = Command::new("sh")
        .arg("-c")
        .arg(format!("{}", cmd))
        .spawn()
    {
        println!("Failed to perform anyrun-powermenu: {}", why);
    }

    HandleResult::Close
}
