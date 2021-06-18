pub struct Dino {
    pub character_class: String,
    pub display_name: String,
    pub aliases: Vec<String>,
    pub enabled: bool,
}

impl Dino {
    pub fn new(character_class: &str, display_name: &str, aliases: Vec<&str>, enabled: bool) -> Self {
        let item = Dino
        {
            character_class: character_class.to_string(),
            display_name: display_name.to_string(),
            aliases: aliases.iter().map(|x| x.to_string()).collect(),
            enabled,
        };
        item
    }

    pub fn list() -> Vec<Dino>  {
        let list = vec![
            Dino::new("Anky", "Anky", vec!["anky", "aids2"], true),
        ];
        list
    }
}
