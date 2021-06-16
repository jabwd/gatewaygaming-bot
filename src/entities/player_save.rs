use serde::{Deserialize, Serialize};

/*
{
    "CharacterClass": "EnterDinoCodeHere",
    "DNA": "",
    "Location_Thenyaw_Island": "X=128935.000 Y=93480.086 Z=-67279.922",
    "Rotation_Thenyaw_Island": "P=0.000000 Y=-121.233009 R=0.000000",
    "Growth": "1.0",
    "Hunger": "9999",
    "Thirst": "9999",
    "Stamina": "9999",
    "Health": "9999",
    "BleedingRate": "0",
    "Oxygen": "40",
    "bGender": false,
    "bIsResting": false,
    "bBrokenLegs": false,
    "ProgressionPoints": "0",
    "ProgressionTier": "1",
    "UnlockedCharacters": "",
    "CameraRotation_Thenyaw_Island": "P=0.000000 Y=-121.232460 R=0.000000",
    "CameraDistance_Thenyaw_Island": "449.998474"
    "SkinPaletteSection1": 40,
    "SkinPaletteSection2": 37,
    "SkinPaletteSection3": 37,
    "SkinPaletteSection4": 24,
    "SkinPaletteSection5": 39,
    "SkinPaletteSection6": 254,
    "SkinPaletteVariation": "6.0"
}
*/

#[derive(Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "CharacterClass")]
    pub character_class: String,
    #[serde(rename = "DNA")]
    pub dna: String,
    #[serde(rename = "Location_Thenyaw_Island")]
    pub location_thenyaw_island: String,
    #[serde(rename = "Rotation_Thenyaw_Island")]
    pub rotation_thenyaw_island: String,
    #[serde(rename = "Growth")]
    pub growth: f32,
    #[serde(rename = "Hunger")]
    pub hunger: i32,
    #[serde(rename = "Thirst")]
    pub thirst: i32,
    #[serde(rename = "Stamina")]
    pub stamina: i32,
    #[serde(rename = "Health")]
    pub health: i32,
    #[serde(rename = "BleedingRate")]
    pub bleeding_rate: f32,
    #[serde(rename = "Oxygen")]
    pub oxygen: i32,
    #[serde(rename = "bGender")]
    pub gender: bool, // male = false, female = true
    #[serde(rename = "bIsResting")]
    pub is_resting: bool,
    #[serde(rename = "bBrokenLegs")]
    pub broken_legs: bool,
    #[serde(rename = "ProgressionPoints")]
    pub progression_points: String,
    #[serde(rename = "ProgressionTier")]
    pub progression_tier: String,
    #[serde(rename = "UnlockedCharacters")]
    pub unlocked_characters: String,
    #[serde(rename = "CameraRotation_Thenyaw_Island")]
    pub camera_rotation_thenyaw_island: String,
    #[serde(rename = "CameraDistance_Thenyaw_Island")]
    pub camera_distance_thenyaw_island: String,
    #[serde(rename = "SkinPaletteSection1")]
    pub skin_palette_section1: i32,
    #[serde(rename = "SkinPaletteSection2")]
    pub skin_palette_section2: i32,
    #[serde(rename = "SkinPaletteSection3")]
    pub skin_palette_section3: i32,
    #[serde(rename = "SkinPaletteSection4")]
    pub skin_palette_section4: i32,
    #[serde(rename = "SkinPaletteSection5")]
    pub skin_palette_section5: i32,
    #[serde(rename = "SkinPaletteSection6")]
    pub skin_palette_section6: i32,
    #[serde(rename = "SkinPaletteVariation")]
    pub skin_palette_variation: String,
}

/*
DiabloAdultS
DiabloJuvS
DiabloHatchS
DryoAdultS
DryoJuvS
DryoHatchS
GalliAdultS
GalliJuvS
GalliHatchS
MaiaAdultS
MaiaJuvS
MaiaHatchS
PachyAdultS
PachyHatchS
PachyJuvS
ParaAdultS
ParaJuvS
ParaHatchS
TrikeAdultS
TrikeSubS
TrikeJuvS
TrikeHatchS

Anky
AnkyJuv
Austro
AustroJuv
Ava
AvaJuv
Camara
Oro
Taco
Puerta
PuertaJuv
Shant
ShantJuv
Stego
StegoJuv
Theri
TheriJuv

Carnivores
AlloAdultS
AlloJuvS
AlloHatchS
CarnoAdultS
CarnoSubS
CarnoJuvS
CarnoHatchS
CeratoAdultS
CeratoJuvS
CeratoHatchS
DiloAdultS
DiloJuvS
DiloHatchS
GigaAdultS
GigaSubS
GigaJuvS
GigaHatchS
SuchoAdultS
SuchoHatchS
SuchoJuvS
RexAdultS
RexSubS
RexJuvS
UtahAdultS
UtahJuvS
UtahHatchS

Acro
AcroJuv
Albert
Bary
BaryJuv
Herrera
HerreraJuv
Spino
SpinoJuv
Velo
*/
