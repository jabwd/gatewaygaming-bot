
pub struct Dino {
    pub character_class: String,
    pub display_name: String,
    pub aliases: Vec<String>,
    pub enabled: bool,
    pub cost: i64,
    pub growth: String,
    pub nesting_roles: Vec<String>,
    pub spawn_roles: Vec<String>,
}

impl Dino {
    pub fn new(character_class: &str, display_name: &str, aliases: Vec<&str>, enabled: bool, cost: i64, growth: &str) -> Self {
        let item = Dino
        {
            character_class: character_class.to_string(),
            display_name: display_name.to_string(),
            aliases: aliases.iter().map(|x| x.to_string()).collect(),
            enabled,
            cost,
            growth: growth.to_string(),
            nesting_roles: vec![],
            spawn_roles: vec![],
        };
        item
    }

    pub fn list() -> Vec<Dino>  {
        let list = vec![
            Dino::new("DiabloAdultS", "Potato", vec!["potat", "potato", "dibble", "diabloceratops", "diablo"], true, 40_000, "1.0"),
            Dino::new("DiabloJuvS", "Potato juvie", vec!["potat-juvie", "dibble-juvie"], false, 0, "1.0"),
            Dino::new("DiabloHatchS", "Potato hatchling", vec!["potat-hatch", "dibble-hatch"], false, 0, "0.5"),

            Dino::new("DryoAdultS", "Dryo", vec!["dryo"], true, 10_000, "1.0"),
            Dino::new("DryoJuvS", "Dryo juvie", vec!["dryo-juvie"], false, 0, "0.5"),
            Dino::new("DryoHatchS", "Dryo hatchling", vec!["dryo-hatch"], false, 0, "0.5"),

            Dino::new("GalliAdultS", "Gallimimus", vec!["galli", "gallimimus"], true, 20_000, "1.0"),
            Dino::new("GalliJuvS", "Galli juvie", vec!["galli-juvie"], false, 0, "0.5"),
            Dino::new("GalliHatchS", "Galli hatchling", vec!["galli-hatch"], false, 0, "0.5"),
            
            Dino::new("MaiaAdultS", "Maiasaura", vec!["maia", "maiasaura"], true, 35_000, "1.0"),
            Dino::new("MaiaJuvS", "Maia juvie", vec!["maia-juvie"], false, 0, "0.5"),
            Dino::new("MaiaHatchS", "Maia hatchling", vec!["maia-hatch"], false, 0, "0.5"),

            Dino::new("PachyAdultS", "Pachycephalosaurus", vec!["pachy", "pachycephalosaurus"], true, 25_000, "1.0"),
            Dino::new("PachyJuvS", "Pachy juvie", vec!["pachy-juvie"], false, 0, "0.5"),
            Dino::new("PachyHatchS", "Pachy hatchling", vec!["pachy-hatch"], false, 0, "0.5"),

            Dino::new("ParaAdultS", "Parasaurolophus", vec!["para", "parasaurolophus"], true, 40_000, "1.0"),
            Dino::new("ParaJuvS", "Para juvie", vec!["para-juvie"], false, 0, "0.5"),
            Dino::new("ParaHatchS", "Para hatchling", vec!["para-hatch"], false, 0, "0.5"),

            Dino::new("TrikeAdultS", "Triceratops", vec!["trike", "triceratops"], true, 70_000, "1.0"),
            Dino::new("TrikeSubS", "Triceratops subadult", vec!["trike-juvie"], true, 35_000, "0.75"),
            Dino::new("TrikeJuvS", "Triceratops juvie", vec!["trike-juvie"], false, 0, "0.5"),
            Dino::new("TrikeHatchS", "Triceratops hatchling", vec!["trike-hatch"], false, 0, "1.0"),

            Dino::new("Anky", "Ankylosaurus", vec!["anky", "ankylosaurus"], true, 45_000, "1.0"),
            Dino::new("AnkyJuv", "Ankylosaurus juvie", vec!["anky-juvie"], true, 0, "0.5"),
            
            Dino::new("Austro", "Austroraptor", vec!["austro", "austroraptor"], true, 5_000, "1.0"),
            Dino::new("AustroJuv", "Austroraptor juvie", vec!["austro-juvie"], true, 0, "0.5"),

            Dino::new("Ava", "Avaceratops", vec!["ava", "avaceratops"], true, 1_000, "1.0"),
            Dino::new("AvaJuv", "Avaceratops juvie", vec!["ava-juvie"], true, 0, "0.5"),

            Dino::new("Camara", "Camarasaurus", vec!["cama", "camarasaurus"], true, 200_000, "1.0"),
            
            Dino::new("Oro", "Orodromeus", vec!["oro", "orodromeus"], true, 1_000, "1.0"),
            
            // TODO: Find the dinosaur display name
            Dino::new("Taco", "Taco", vec!["taco", ""], true, 1_000, "1.0"),

            Dino::new("Puerta", "Puertasaurus", vec!["pue", "puerta", "puertasaurus"], true, 400_000, "1.0"),
            Dino::new("PuertaJuv", "Puertasaurus juvie", vec!["pue-juvie"], true, 400_000, "1.0"),

            Dino::new("Shant", "Shantungosaurus", vec!["shant", "shantungosaurus"], true, 100_000, "1.0"),
            Dino::new("ShantJuv", "Shantunogausurs juvie", vec!["shant-juvie"], true, 0, "0.4"),

            Dino::new("Stego", "Stegosaurus", vec!["stego", "stegosaurus"], true, 50_000, "1.0"),
            Dino::new("StegoJuv", "Stegosaurus juvie", vec!["stego-juvie"], true, 0, "0.4"),

            Dino::new("Theri", "Therizinosaurus", vec!["theri", "therizinosaurus"], true, 60_000, "1.0"),
            Dino::new("TheriJuv", "Therizinosaurus juvie", vec!["theri-juvie"], true, 0, "0.4"),

            Dino::new("AlloAdultS", "Allosaurus", vec!["alo", "allo", "allosaurus"], true, 50_000, "1.0"),
            Dino::new("AlloJuvS", "Allosaurus juvie", vec!["allo-juvie"], true, 0, "0.5"),
            Dino::new("AlloHatchS", "Allosaurus hatch", vec!["allo-hatch"], true, 0, "0.5"),

            Dino::new("CarnoAdultS", "Carnotaurus", vec!["carno", "carnotaurus"], true, 35_000, "1.0"),
            Dino::new("CarnoSubS", "Carno sub", vec!["carno-sub"], false, 0, "0.5"),
            Dino::new("CarnoJuvS", "Carno juvie", vec!["carno-juvie"], true, 0, "0.5"),
            Dino::new("CarnoHatchS", "Carno hatch", vec!["carno-hatch"], true, 0, "0.5"),

            Dino::new("CeratoAdultS", "Ceratosaurus", vec!["cerato", "ceratosaurus"], true, 30_000, "1.0"),
            Dino::new("CeratoJuvS", "Hell spawn juvie", vec!["dino-missile", "cerato-juvie"], true, 15_000, "1.0"),
            Dino::new("CeratoHatchS", "Ceratosaurus hatch", vec!["cerato-hatch"], true, 0, "0.5"),

            Dino::new("DiloAdultS", "Dilophosaurus", vec!["dilo", "dilophosaurus"], true, 20_000, "1.0"),
            Dino::new("DiloJuvS", "Dilophosaurus juvie", vec!["dilo-juvie"], true, 0, "0.6"),
            Dino::new("DiloHatchS", "Dilophosaurus hatch", vec!["dilo-hatch"], true, 0, "0.6"),

            // 0.6 growth for fresh adult
            Dino::new("UtahAdultS", "Utahraptor", vec!["utah", "utahraptor"], true, 20_000, "1.0"),
            Dino::new("UtahJuvS", "Utahraptor juvie", vec!["utah-juvie"], true, 0, "0.6"),
            Dino::new("UtahHatchS", "Utahraptor hatch", vec!["utah-hatch"], true, 0, "0.5"),

            Dino::new("GigaAdultS", "Giganotosaurus", vec!["giga", "giganotosaurus"], true, 65_000, "1.0"),
            Dino::new("GigaSubS", "Giganotosaurus sub", vec!["giga-sub"], true, 30_000, "0.7"),
            Dino::new("GigaJuvS", "Giganotosaurus juvie", vec!["giga-juvie"], true, 0, "0.5"),
            Dino::new("GigaHatchS", "Giganotosaurus hatch", vec!["giga-hatch"], true, 0, "0.4"),

            // .8 fresh adult
            Dino::new("SuchoAdultS", "Suchomimus", vec!["sucho", "suchomimus"], true, 50_000, "1.0"),
            Dino::new("SuchoJuvS", "Suchomimus juvie", vec!["sucho-juvie"], true, 0, "0.5"),
            Dino::new("SuchoHatchS", "Suchomimus hatch", vec!["sucho-hatch"], true, 0, "0.5"),

            Dino::new("RexAdultS", "Tyrannosaurus rex", vec!["trex", "rex", "tyrannosaurus"], true, 70_000, "1.0"),
            Dino::new("RexSubS", "Tyrannosaurus sub", vec!["rex-sub"], true, 35_000, "0.78"),
            Dino::new("RexJuvS", "Tyrannosaurus juvie", vec!["rex-juvie"], true, 0, "0.5"),
            Dino::new("RexHatchS", "Tyrannosaurus hatch", vec!["rex-hatch"], true, 0, "0.5"),

            // 0.6 fresh adult
            Dino::new("Acro", "Acrocanthosaurus", vec!["acro", "acrocanthosaurus"], true, 60_000, "1.0"),
            Dino::new("AcroJuv", "Acro juvie", vec!["acro-juvie"], true, 0, "0.1"),

            Dino::new("Albert", "Albertosaurus", vec!["albert-free", "alberto-free", "albertosaurus-free"], true, 0, "0.4"),
            Dino::new("Albert", "Albertosaurus", vec!["albert", "alberto", "albertosaurus"], true, 50_000, "1.0"),

            Dino::new("Bary", "Baryonyx", vec!["bary", "baryonyx"], true, 25_000, "1.0"),
            Dino::new("BaryJuv", "Baryonyx juvie", vec!["bary-juvie"], true, 0, "0.5"),

            Dino::new("Herrera", "Herrerasaurus", vec!["herrera", "herrerasaurus"], true, 5_000, "1.0"),
            Dino::new("HerreraJuv", "Herrerasaurus juvie", vec!["herrera-juvie"], true, 0, "0.5"),
            Dino::new("Spino", "Spinosaurus", vec!["spino", "spinosaurus"], true, 100_000, "1.0"),
            Dino::new("SpinoJuv", "Spinosaurus juvie", vec!["spino-juvie"], true, 0, "0.4"),

            Dino::new("Velo", "Velociraptor", vec!["velo", "velociraptor"], true, 1_000, "1.0")
        ];

        list
    }
}
