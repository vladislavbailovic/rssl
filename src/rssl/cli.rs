use super::model;

pub struct Cli {
    source: model::Source,
}

pub fn parse(params: Vec<String>) -> Cli {
    let fallback = &Vec::new();
    let (_bin, args) = params
        .split_first()
        .unwrap_or((&"rssl".to_string(), fallback));

    let mut path = String::from(".");
    for arg in args {
        // TODO: parse args
        // ... else:
        path = arg.as_str().to_string();
    }
    Cli {
        source: model::Source::Filelist(path),
    }
}

impl Cli {
    pub fn source(&self) -> &model::Source {
        &self.source
    }
}
