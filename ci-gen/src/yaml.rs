#[derive(Clone)]
pub enum Yaml {
    String(String),
    List(Vec<Yaml>),
    Map(Vec<(String, Yaml)>),
}

impl From<&Yaml> for Yaml {
    fn from(y: &Yaml) -> Self {
        y.clone()
    }
}

impl From<String> for Yaml {
    fn from(s: String) -> Self {
        Yaml::String(s)
    }
}

impl From<&str> for Yaml {
    fn from(s: &str) -> Self {
        Yaml::String(s.to_owned())
    }
}

impl From<&&str> for Yaml {
    fn from(s: &&str) -> Self {
        Yaml::String((*s).to_owned())
    }
}

impl<T: Into<Yaml>> From<Vec<T>> for Yaml {
    fn from(v: Vec<T>) -> Self {
        Yaml::List(v.into_iter().map(|t| t.into()).collect())
    }
}

impl Yaml {
    pub fn map<K: Into<String>, V: Into<Yaml>, E: IntoIterator<Item = (K, V)>>(entries: E) -> Yaml {
        Yaml::Map(
            entries
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }

    pub fn list<V: Into<Yaml>, E: IntoIterator<Item = V>>(values: E) -> Yaml {
        Yaml::List(values.into_iter().map(|v| v.into()).collect())
    }

    pub fn string<S: Into<String>>(s: S) -> Yaml {
        Yaml::String(s.into())
    }
}

#[derive(Default)]
pub struct YamlWriter {
    pub buffer: String,
    indent: u32,
    minus: MinusState,
}

#[derive(Eq, PartialEq)]
enum MinusState {
    No,
    Yes,
    Already,
}

impl Default for MinusState {
    fn default() -> Self {
        MinusState::No
    }
}

impl YamlWriter {
    pub fn write_line(&mut self, line: &str) {
        if line.is_empty() {
            self.buffer.push_str("\n");
        } else {
            for _ in 0..self.indent {
                self.buffer.push_str("    ");
            }

            match self.minus {
                MinusState::No => {}
                MinusState::Yes => {
                    self.buffer.push_str("- ");
                    self.minus = MinusState::Already;
                }
                MinusState::Already => {
                    self.buffer.push_str("  ");
                }
            }

            self.buffer.push_str(line);
            self.buffer.push_str("\n");
        }
    }

    pub fn write_yaml(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::String(s) => {
                self.write_line(s);
            }
            Yaml::List(l) => {
                for x in l {
                    assert!(self.minus == MinusState::No);
                    self.minus = MinusState::Yes;
                    self.write_yaml(x);
                    assert!(self.minus != MinusState::No);
                    self.minus = MinusState::No;
                }
            }
            Yaml::Map(m) => {
                for (k, v) in m {
                    match v {
                        Yaml::String(v) => {
                            self.write_line(&format!("{}: {}", k, v));
                        }
                        _ => {
                            self.write_line(&format!("{}:", k));
                            self.indent += 1;
                            self.write_yaml(v);
                            self.indent -= 1;
                        }
                    }
                }
            }
        }
    }
}
