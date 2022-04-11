use std::fmt;

use crate::yaml::Yaml;

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Env {
    WindowsLatest,
    UbuntuLatest,
    MacosLatest,
}

impl Default for Env {
    fn default() -> Self {
        Env::UbuntuLatest
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Env::WindowsLatest => write!(f, "windows-latest"),
            Env::UbuntuLatest => write!(f, "ubuntu-latest"),
            Env::MacosLatest => write!(f, "macos-latest"),
        }
    }
}

/// Github workflow step
#[derive(Default)]
pub struct Step {
    pub name: String,
    pub uses: Option<String>,
    pub with: Option<Yaml>,
    pub run: Option<String>,
    pub shell: Option<String>,
    pub env: Vec<(String, String)>,
}

impl Step {
    pub fn uses(name: &str, uses: &str) -> Step {
        Step {
            name: name.to_owned(),
            uses: Some(uses.to_owned()),
            ..Default::default()
        }
    }

    pub fn uses_with(name: &str, uses: &str, with: Yaml) -> Step {
        Step {
            name: name.to_owned(),
            uses: Some(uses.to_owned()),
            with: Some(with),
            ..Default::default()
        }
    }

    pub fn uses_env_with(name: &str, uses: &str, env: &[(&str, &str)], with: Yaml) -> Step {
        Step {
            name: name.to_owned(),
            uses: Some(uses.to_owned()),
            env: env
                .into_iter()
                .map(|(k, v)| (String::from(*k), String::from(*v)))
                .collect(),
            with: Some(with),
            ..Default::default()
        }
    }

    pub fn run(name: &str, run: &str) -> Step {
        Step {
            name: name.to_owned(),
            run: Some(run.to_owned()),
            shell: Some("bash".to_owned()),
            ..Default::default()
        }
    }
}

impl Step {
    pub fn env(mut self, name: &str, value: &str) -> Self {
        self.env.push((name.to_owned(), value.to_owned()));
        self
    }
}

impl Into<Yaml> for Step {
    fn into(self) -> Yaml {
        let Step {
            name,
            uses,
            with,
            run,
            shell,
            env,
        } = self;
        let mut entries = Vec::new();
        entries.push(("name", Yaml::string(name)));
        if let Some(uses) = uses {
            entries.push(("uses", Yaml::string(uses)));
        }
        if let Some(with) = with {
            entries.push(("with", with));
        }
        if let Some(run) = run {
            entries.push(("run", Yaml::string(run)));
        }
        if let Some(shell) = shell {
            entries.push(("shell", Yaml::string(shell)));
        }
        if !env.is_empty() {
            entries.push(("env", Yaml::map(env)));
        }
        Yaml::map(entries)
    }
}

#[derive(Default)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub runs_on: Env,
    pub steps: Vec<Step>,
    pub timeout_minutes: Option<u64>,
    pub env: Vec<(String, String)>,
}

impl Job {
    #[allow(dead_code)]
    pub fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }
}

impl Into<(String, Yaml)> for Job {
    fn into(self) -> (String, Yaml) {
        assert!(!self.id.is_empty());
        let mut entries = vec![
            ("name", Yaml::string(self.name)),
            ("runs-on", Yaml::string(format!("{}", self.runs_on))),
        ];
        if let Some(timeout_minutes) = self.timeout_minutes {
            entries.push((
                "timeout-minutes",
                Yaml::string(format!("{}", timeout_minutes)),
            ));
        }
        if !self.env.is_empty() {
            entries.push(("env", Yaml::map(self.env)));
        }
        entries.push(("steps", Yaml::list(self.steps)));
        (self.id, Yaml::map(entries))
    }
}
