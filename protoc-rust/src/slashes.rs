pub(crate) enum Slashes {
    Unix,
    Windows,
}

impl Slashes {
    pub fn here() -> Slashes {
        if cfg!(windows) {
            Slashes::Windows
        } else if cfg!(unix) {
            Slashes::Unix
        } else {
            panic!("Unknown operating system")
        }
    }

    fn slashes(&self) -> &'static [char] {
        match self {
            &Slashes::Unix => &['/'],
            &Slashes::Windows => &['/', '\\'],
        }
    }

    fn _is_slash(&self, c: char) -> bool {
        self.slashes().contains(&c)
    }

    pub fn norm_path(&self, path: &str) -> String {
        match self {
            &Slashes::Unix => path.to_owned(),
            &Slashes::Windows => path.replace('\\', "/"),
        }
    }

    fn remove_dot_slash<'a>(&self, path: &'a str) -> &'a str {
        if path == "." {
            ""
        } else if path.starts_with(".") {
            let mut temp_path = &path[1..];
            let mut at_least_one_slash = false;
            while temp_path.starts_with(self.slashes()) {
                temp_path = &temp_path[1..];
                at_least_one_slash = true;
            }
            if at_least_one_slash {
                temp_path
            } else {
                path
            }
        } else {
            path
        }
    }

    /// Remove leading ./ from path
    pub fn remove_dot_slashes<'a>(&self, mut path: &'a str) -> &'a str {
        loop {
            let new_path = self.remove_dot_slash(path);
            if new_path == path {
                return new_path;
            }
            path = new_path;
        }
    }
}
