use super::model::FilteredList;

pub enum Source {
    Static(String),
    Filelist(String),
    Command(String),
}
impl Source {
    pub fn load(&self, title: &str) -> FilteredList {
        match self {
            Source::Static(content) => {
                let items: Vec<String> = content.split('\n').map(String::from).collect();
                FilteredList::new(title, items)
            }
            Source::Filelist(path) => {
                let filelist = filelist(path).unwrap_or_else(|e| {
                    panic!("Error listing directory: [{}] because: {}", path, e);
                });
                let src = Source::Static(filelist);
                src.load(title)
            }
            Source::Command(cmd) => {
                let res = command(cmd).unwrap_or_else(|e| {
                    panic!("Unable to run command: [{}] because: {}", cmd, e);
                });
                let src = Source::Static(res);
                src.load(title)
            }
        }
    }
}

fn filelist(path: &str) -> Result<String, std::io::Error> {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let item = String::from(entry.path().to_string_lossy());
        result.push(item);
    }
    Ok(result.join("\n"))
}

fn command(command: &str) -> Result<String, std::io::Error> {
    use std::process::Command;
    let parts: Vec<&str> = command.split(' ').collect();
    let mut command = Command::new(parts[0]);
    if parts.len() > 1 {
        for arg in parts[1..].iter() {
            command.arg(arg);
        }
    }
    let command = command.output()?;
    let result = String::from_utf8(command.stdout).expect("Unable to parse stdout");
    if result.is_empty() {
        return Ok(String::from_utf8(command.stderr).expect("Unable to parse stderr"));
    }
    Ok(result)
}
