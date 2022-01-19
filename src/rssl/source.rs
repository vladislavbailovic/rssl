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
                let filelist = filelist(path);
                let src = Source::Static(filelist);
                src.load(title)
            }
            Source::Command(cmd) => {
                let res = command(cmd);
                let src = Source::Static(res);
                src.load(title)
            }
        }
    }
}

fn filelist(path: &str) -> String {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(path).expect("Unable to list directory") {
        result.push(entry.expect("Invalid entry").path().into_os_string().into_string().unwrap());
    }
    result.join("\n")
}

fn command(command: &str) -> String {
    use std::process::Command;
    let command = Command::new("find")
        .arg(command)
        .output()
        .expect("Command execution failed");
    let result = String::from_utf8(command.stdout).expect("Invalid stdout");
    if result.is_empty() {
        return String::from_utf8(command.stderr).expect("Invalid stderr");
    }
    result
}
