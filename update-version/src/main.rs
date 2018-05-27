use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt::Write as fmt_Write;

extern crate tempdir;
extern crate regex;

extern crate toml;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
struct TomlWorkspace {
    members: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
struct TomlPackage {
    name: String,
    version: String,
    publish: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct DetailedTomlDependency {
    version: Option<String>,
    path: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TomlDependency {
    Simple(String),
    Detailed(DetailedTomlDependency),
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
struct TomlManifest {
    package: Option<TomlPackage>,
    workspace: Option<TomlWorkspace>,
    dependencies: Option<BTreeMap<String, TomlDependency>>,
}


fn find_repo_root() -> PathBuf {
    let candidates = &[".", ".."];
    for dir in candidates {
        let dir = Path::new(dir).to_owned();
        let mut file = dir.clone();
        file.push("protobuf-test");
        if file.exists() {
            return dir;
        }
    }
    panic!("cannot find repository root");
}

/// Read `Cargo.toml` from specified directory.
fn read_crate_manifest(crate_path: &Path) -> TomlManifest {
    assert!(crate_path.is_dir());

    let mut cargo_toml = crate_path.to_owned();
    cargo_toml.push("Cargo.toml");

    let content = fs::read_to_string(&cargo_toml).expect(&format!("read {:?}", cargo_toml));

    toml::from_str(&content)
        .expect(&format!("parse toml {:?}", cargo_toml))
}

fn find_workspace_members(repo_root: &Path) -> Vec<String> {
    read_crate_manifest(repo_root).workspace.expect("workspace").members
}


#[derive(Debug)]
struct ParsedDependency {
    name: String,
    path: String,
}


#[derive(Debug)]
struct ParsedMember {
    /// Package name
    name: String,
    /// Path in the workspace
    member: String,
    /// Internal dep package names
    internal_deps: Vec<ParsedDependency>,
}


fn topo_sort(members: &mut [ParsedMember]) {
    'again: loop {
        for i in 0..members.len() {
            for j in i..members.len() {
                if i == j {
                    continue;
                }

                if members[i].internal_deps.iter().any(|d| d.name == members[j].name) {
                    members.swap(i, j);
                    // Native algorithm
                    continue 'again;
                }
            }
        }
        break;
    }
}


fn patch_crate(repo_root: &Path, member: &ParsedMember, new_version: &str) {
    let mut manifest_path = repo_root.to_owned();
    manifest_path.push(&member.member);
    manifest_path.push("Cargo.toml");

    let read = File::open(&manifest_path).expect(&format!("open manifest {:?}", manifest_path));
    let read = BufReader::new(read);

    #[derive(Debug, Eq, PartialEq)]
    enum WhereWeAre {
        Init,
        Package,
        Dependencies,
        Other,
    }

    let mut where_we_are = WhereWeAre::Init;

    let mut output = String::new();

    let mut version_patched = false;
    let mut internal_deps_seen = HashSet::new();

    for line in read.lines() {
        let line = line.expect("line");

        let mut line_written = false;

        if line == "[package]" {
            where_we_are = WhereWeAre::Package;
        } else if line == "[dependencies]" {
            where_we_are = WhereWeAre::Dependencies;
        } else if line.starts_with("[") {
            where_we_are = WhereWeAre::Other;
        } else {
            match where_we_are {
                WhereWeAre::Package => {
                    if line.starts_with("version =") {
                        assert!(!version_patched, "in {}", member.name);
                        version_patched = true;

                        writeln!(output, "version = \"{}\"", new_version).expect("write");
                        line_written = true;
                    }
                }
                WhereWeAre::Dependencies => {
                    for dep in &member.internal_deps {
                        if line.starts_with(&format!("{} ", dep.name)) {
                            let inserted = internal_deps_seen.insert(dep.name.clone());
                            assert!(inserted, "dep more than once in {}", member.name);

                            writeln!(
                                output,
                                "{} = {{ path = \"{}\", version = \"={}\" }}",
                                dep.name, dep.path, new_version)
                                .expect("write");
                            line_written = true;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        if !line_written {
            writeln!(output, "{}", line).expect("write");
        }
    }

    assert!(version_patched, "in {}", member.name);
    assert_eq!(member.internal_deps.len(), internal_deps_seen.len(), "in {}", member.name);

    fs::write(&manifest_path, &output).expect("write patched manifest back");
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert_eq!(1, args.len());
    let new_version = &args[0];

    let repo_root = find_repo_root();

    let members = find_workspace_members(&repo_root);

    let mut publish_members = Vec::new();

    for member in members {
        let mut crate_path = repo_root.to_owned();
        crate_path.push(&member);
        let manifest = read_crate_manifest(&crate_path);
        let package = manifest.package
            .expect(&format!("package in {}", member));
        let dependencies = manifest.dependencies
            .expect(&format!("dependencies in {}", member));
        if package.publish == Some(false) {
            continue;
        }
        if package.version == "0.0.0" {
            panic!("version must be 0.0.0 is !publish for {}", member);
        }
        if package.version == "0.1.0" {
            panic!("default version 1.0.0 in {}", member);
        }

        let name = package.name;

        let mut internal_deps = Vec::new();

        for (name, dep) in dependencies {
            if let TomlDependency::Detailed(d) = dep {
                let path = match d.path {
                    Some(path) => path,
                    None => {
                        // Not internal dependency
                        continue
                    },
                };
                let version = d.version.expect(&format!("version in dep of {}", member));
                assert!(version.starts_with("="), "dep version must start with =: {}", member);
                internal_deps.push(ParsedDependency {
                    name,
                    path,
                });
            }
        }

        publish_members.push(ParsedMember {
            name,
            member,
            internal_deps,
        });
    }

    topo_sort(&mut publish_members);

    for member in &publish_members {
        patch_crate(&repo_root, member, new_version);
    }

    for member in &publish_members {
        println!("cargo publish --manifest-path={}/Cargo.toml", member.member);
    }
    println!("git tag v{}", new_version);
}
