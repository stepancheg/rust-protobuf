use crate::yaml::Yaml;
use std::fmt;

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
pub struct Step(pub Yaml);

impl Step {
    pub fn uses(name: &str, uses: &str) -> Step {
        Step(Yaml::map(vec![("name", name), ("uses", uses)]))
    }

    pub fn uses_with(name: &str, uses: &str, with: Yaml) -> Step {
        Step(Yaml::map(vec![
            ("name", Yaml::string(name)),
            ("uses", Yaml::string(uses)),
            ("with", with),
        ]))
    }

    pub fn uses_env_with(name: &str, uses: &str, env: &[(&str, &str)], with: Yaml) -> Step {
        Step(Yaml::map(vec![
            ("name", Yaml::string(name)),
            ("uses", Yaml::string(uses)),
            ("env", Yaml::map(env.to_owned())),
            ("with", with),
        ]))
    }

    pub fn run(name: &str, run: &str) -> Step {
        Step(Yaml::map(vec![
            ("name", name),
            ("run", run),
            ("shell", "bash"),
        ]))
    }
}

impl Into<Yaml> for Step {
    fn into(self) -> Yaml {
        self.0
    }
}

#[derive(Default)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub runs_on: Env,
    pub steps: Vec<Step>,
    pub env: Vec<(String, String)>,
}

impl Job {
    pub fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }
}

impl Into<(String, Yaml)> for Job {
    fn into(self) -> (String, Yaml) {
        let mut entries = vec![
            ("name", Yaml::string(self.name)),
            ("runs-on", Yaml::string(format!("{}", self.runs_on))),
        ];
        if !self.env.is_empty() {
            entries.push(("env", Yaml::map(self.env)));
        }
        entries.push(("steps", Yaml::list(self.steps)));
        (self.id, Yaml::map(entries))
    }
}
