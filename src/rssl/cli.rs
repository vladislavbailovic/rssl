use super::source::Source;

pub struct Cli {
    source: Source,
}

pub fn parse(params: Vec<String>) -> Cli {
    let fallback = &Vec::new();
    let (_bin, args) = params
        .split_first()
        .unwrap_or((&"rssl".to_string(), fallback));

    let source = if args.is_empty() {
        Source::Filelist(".".to_string())
    } else {
        match args[0].as_str() {
            "-c" if args.len() > 1 => Source::Command(args[1].as_str().to_string()),
            "-c" => panic!("Missing command"),
            _ => Source::Filelist(args[0].as_str().to_string()),
        }
    };
    Cli { source }
}

impl Cli {
    pub fn source(&self) -> &Source {
        &self.source
    }
}
