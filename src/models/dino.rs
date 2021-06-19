
pub struct Dino {
    pub character_class: String,
    pub display_name: String,
    pub aliases: Vec<String>,
    pub enabled: bool,
    pub cost: i32,
}

impl Dino {
    pub fn new(character_class: &str, display_name: &str, aliases: Vec<&str>, enabled: bool) -> Self {
        let item = Dino
        {
            character_class: character_class.to_string(),
            display_name: display_name.to_string(),
            aliases: aliases.iter().map(|x| x.to_string()).collect(),
            enabled,
            cost: 0,
        };
        item
    }

    pub fn list() -> Vec<Dino>  {
        let list = vec![
            Dino::new("DiabloAdultS", "Potato", vec!["potat", "potato", "dibble", "diabloceratops", "diablo"], true),
            Dino::new("DiabloJuvS", "Potato juvie", vec!["potat-juvie", "dibble-juvie"], true),
            Dino::new("DiabloHatchS", "Potato hatchling", vec!["potat-hatch", "dibble-hatch"], true),

            Dino::new("DryoAdultS", "Dryo", vec!["dryo"], true),
            Dino::new("DryoJuvS", "Dryo juvie", vec!["dryo-juvie"], true),
            Dino::new("DryoHatchS", "Dryo hatchling", vec!["dryo-hatch"], true),

            Dino::new("GalliAdultS", "Gallimimus", vec!["galli", "gallimimus"], true),
            Dino::new("GalliJuvS", "Galli juvie", vec!["galli-juvie"], true),
            Dino::new("GalliHatchS", "Galli hatchling", vec!["galli-hatch"], true),
            
            Dino::new("MaiaAdultS", "Maiasaura", vec!["maia", "maiasaura"], true),
            Dino::new("MaiaJuvS", "Maia juvie", vec!["maia-juvie"], true),
            Dino::new("MaiaHatchS", "Maia hatchling", vec!["maia-hatch"], true),

            Dino::new("PachyAdultS", "Pachycephalosaurus", vec!["pachy", "pachycephalosaurus"], true),
            Dino::new("PachyJuvS", "Pachy juvie", vec!["pachy-juvie"], true),
            Dino::new("PachyHatchS", "Pachy hatchling", vec!["pachy-hatch"], true),

            Dino::new("ParaAdultS", "Parasaurolophus", vec!["para", "parasaurolophus"], true),
            Dino::new("ParaJuvS", "Para juvie", vec!["para-juvie"], true),
            Dino::new("ParaHatchS", "Para hatchling", vec!["para-hatch"], true),

            Dino::new("TrikeAdultS", "Triceratops", vec!["trike", "triceratops"], true),
            Dino::new("TrikeSubS", "Triceratops subadult", vec!["trike-juvie"], true),
            Dino::new("TrikeJuvS", "Triceratops juvie", vec!["trike-juvie"], true),
            Dino::new("TrikeHatchS", "Triceratops hatchling", vec!["trike-hatch"], true),

            Dino::new("Anky", "Ankylosaurus", vec!["anky", "ankylosaurus"], true),
            Dino::new("AnkyJuv", "Ankylosaurus juvie", vec!["anky-juvie"], true),
            
            Dino::new("Austro", "Austroraptor", vec!["austro", "austroraptor"], true),
            Dino::new("AustroJuv", "Austroraptor juvie", vec!["austro-juvie"], true),

            Dino::new("Ava", "Avaceratops", vec!["ava", "avaceratops"], true),
            Dino::new("AvaJuv", "Avaceratops juvie", vec!["ava-juvie"], true),

            Dino::new("Camara", "Camarasaurus", vec!["cama", "camarasaurus"], true),
            
            Dino::new("Oro", "Orodromeus", vec!["oro", "orodromeus"], true),
            
            // TODO: Find the dinosaur display name
            Dino::new("Taco", "Taco", vec!["taco", ""], true),

            Dino::new("Puerta", "Puertasaurus", vec!["pue", "puerta", "puertasaurus"], true),
            Dino::new("PuertaJuv", "Puertasaurus juvie", vec!["pue-juvie"], true),

            Dino::new("Shant", "Shantungosaurus", vec!["shant", "shantungosaurus"], true),
            Dino::new("ShantJuv", "Shantunogausurs juvie", vec!["shant-juvie"], true),

            Dino::new("Stego", "Stegosaurus", vec!["stego", "stegosaurus"], true),
            Dino::new("StegoJuv", "Stegosaurus juvie", vec!["stego-juvie"], true),

            Dino::new("Theri", "Therizinosaurus", vec!["theri", "therizinosaurus"], true),
            Dino::new("TheriJuv", "Therizinosaurus juvie", vec!["theri-juvie"], true),

            Dino::new("AlloAdultS", "Allosaurus", vec!["alo", "allo", "allosaurus"], true),
            Dino::new("AlloJuvS", "Allosaurus juvie", vec!["allo-juvie"], true),
            Dino::new("AlloHatchS", "Allosaurus hatch", vec!["allo-hatch"], true),

            Dino::new("CarnoAdultS", "Carnotaurus", vec!["carno", "carnotaurus"], true),
            Dino::new("CarnoSubS", "Carno sub", vec!["carno-sub"], true),
            Dino::new("CarnoJuvS", "Carno juvie", vec!["carno-juvie"], true),
            Dino::new("CarnoHatchS", "Carno hatch", vec!["carno-hatch"], true),

            Dino::new("CeratoAdultS", "Ceratosaurus", vec!["cerato", "ceratosaurus"], true),
            Dino::new("CeratoJuvS", "Hell spawn juvie", vec!["dino-missile", "cerato-juvie"], true),
            Dino::new("CeratoHatchS", "Ceratosaurus hatch", vec!["cerato-hatch"], true),

            Dino::new("DiloAdultS", "Dilophosaurus", vec!["dilo", "dilophosaurus"], true),
            Dino::new("DiloJuvS", "Dilophosaurus juvie", vec!["dilo-juvie"], true),
            Dino::new("DiloHatchS", "Dilophosaurus hatch", vec!["dilo-hatch"], true),

            Dino::new("UtahAdultS", "Utahraptor", vec!["utah", "utahraptor"], true),
            Dino::new("UtahJuvS", "Utahraptor juvie", vec!["utah-juvie"], true),
            Dino::new("UtahHatchS", "Utahraptor hatch", vec!["utah-hatch"], true),

            Dino::new("GigaAdultS", "Giganotosaurus", vec!["giga", "giganotosaurus"], true),
            Dino::new("GigaSubS", "Giganotosaurus sub", vec!["giga-sub"], true),
            Dino::new("GigaJuvS", "Giganotosaurus juvie", vec!["giga-juvie"], true),
            Dino::new("GigaHatchS", "Giganotosaurus hatch", vec!["giga-hatch"], true),
            
            Dino::new("SuchoAdultS", "Suchomimus", vec!["sucho", "suchomimus"], true),
            Dino::new("SuchoJuvS", "Suchomimus juvie", vec!["sucho-juvie"], true),
            Dino::new("SuchoHatchS", "Suchomimus hatch", vec!["sucho-hatch"], true),

            Dino::new("RexAdultS", "Tyrannosaurus rex", vec!["trex", "rex", "tyrannosaurus"], true),
            Dino::new("RexSubS", "Tyrannosaurus sub", vec!["rex-sub"], true),
            Dino::new("RexJuvS", "Tyrannosaurus juvie", vec!["rex-juvie"], true),
            Dino::new("RexHatchS", "Tyrannosaurus hatch", vec!["rex-hatch"], true),

            Dino::new("Acro", "Acrocanthosaurus", vec!["acro", "acrocanthosaurus"], true),
            Dino::new("AcroJuv", "Acro juvie", vec!["acro-juvie"], true),
            
            Dino::new("Albert", "Albertosaurus", vec!["alberto", "albertosaurus"], true),

            Dino::new("Bary", "Baryonyx", vec!["bary", "baryonyx"], true),
            Dino::new("BaryJuv", "Baryonyx juvie", vec!["bary-juvie"], true),

            Dino::new("Herrera", "Herrerasaurus", vec!["herrera", "herrerasaurus"], true),
            Dino::new("HerreraJuv", "Herrerasaurus juvie", vec!["herrera-juvie"], true),
            Dino::new("Spino", "Spinosaurus", vec!["spino", "spinosaurus"], true),
            Dino::new("SpinoJuv", "Spinosaurus juvie", vec!["spino-juvie"], true),

            Dino::new("Velo", "Velociraptor", vec!["velo", "velociraptor"], true)
        ];

        list
    }
}
