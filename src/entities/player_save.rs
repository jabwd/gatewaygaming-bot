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

pub struct Player {
    pub character_class: String,

    pub gender: bool,
    pub is_resting: bool,
    pub broken_legs: bool,

    pub progression_points: String,
    pub progression_ier: String,
    pub unlocked_characters: String,

    pub camera_rotation_thenyaw_island: String,
    pub camera_distance_thenyaw_island: String,

    pub skin_palette_section1: i32,
    pub skin_palette_section2: i32,
    pub skin_palette_section3: i32,
    pub skin_palette_section4: i32,
    pub skin_palette_section5: i32,
    pub skin_palette_section6: i32,
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
