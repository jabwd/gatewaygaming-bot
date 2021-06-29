pub struct Rotation {
  pub pitch: String,
  pub yaw: String,
  pub roll: String,
}

pub struct Location {
  pub x: String,
  pub y: String,
  pub z: String,
}

pub struct Teleport {
  pub label: String,
  pub location: Location,
  pub rotation: Rotation,
}

impl Teleport {
  pub fn new(label: &str, x: &str, y: &str, z: &str, pitch: &str, yaw: &str, roll: &str) -> Teleport {
    let location = Location
    {
      x: x.to_string(),
      y: y.to_string(),
      z: z.to_string(),
    };

    let rotation = Rotation
    {
      pitch: pitch.to_string(),
      yaw: yaw.to_string(),
      roll: roll.to_string(),
    };

    let tp = Teleport
    {
      label: label.to_string(),
      location,
      rotation,
    };

    tp
  }

  pub fn tp_locations() -> Vec<Teleport> {
    let list = vec![
      Teleport::new("greatfalls", "-253624.859", "512152.906", "-25232.252", "0.000000", "130.954971", "0.000000"),
      Teleport::new("greatfalls", "-250362.094", "556116.5", "-20790.934", "0.000000", "130.954971", "0.000000"),

      Teleport::new("wash", "-378757.844", "301271.250", "-63352.480", "0.000000", "102.12200", "0.000000"),
      Teleport::new("wash", "-370980.625", "341607.906", "-64949.922", "0.000000", "102.1200", "0.000000"),

      Teleport::new("landbridge", "-351903.312", "67331.461", "-73358.25", "0.000000", "102.1200", "0.000000"),

      Teleport::new("twins", "-463301.094", "-202647.391", "-71612.938", "0.000000", "102.1200", "0.000000"),
      Teleport::new("twins", "-397790.938", "-200770.906", "-69647.273", "0.000000", "102.1200", "0.000000"),

      Teleport::new("titan", "61883.891", "-160841.297", "-62650.688", "0.000000", "102.1200", "0.000000"),
      Teleport::new("titan", "22028.447", "-169786.203", "-65041.602", "0.000000", "102.1200", "0.000000"),
      Teleport::new("titan", "-62766.828", "-239913.812", "-65070.922", "0.000000", "102.1200", "0.000000"),
      Teleport::new("titan", "-63213.469", "-302744.25", "-63669.535", "0.000000", "102.1200", "0.000000"),

      Teleport::new("sinkhole", "-196926.719", "-8274.218", "-71082.219", "0.000000", "102.1200", "0.000000"),
      Teleport::new("sinkhole", "-217396.312", "-24415.045", "-62561.25", "0.000000", "102.1200", "0.000000"),

      Teleport::new("eastswamp", "181224.312", "87525.977", "-74756.484", "0.000000", "102.1200", "0.000000"),
      Teleport::new("eastswamp", "228425.938", "83383.398", "-74882.508", "0.000000", "102.1200", "0.000000"),

      Teleport::new("murky", "326736.5", "-97513.727", "-25714.803", "0.000000", "102.1200", "0.000000"),
      Teleport::new("murky", "305660.906", "-122063.469", "-25952.006", "0.000000", "102.1200", "0.000000"),
      Teleport::new("murky", "299523.375", "-108436", "-27686.732", "0.000000", "102.1200", "0.000000"),

      Teleport::new("port", "400536.188", "269163.625", "-72149.25", "0.000000", "102.1200", "0.000000"),
      Teleport::new("port", "400997.656", "255636.422", "-72390.93", "0.000000", "102.1200", "0.000000"),
      Teleport::new("port", "411201.594", "243300.125", "-72664.141", "0.000000", "102.1200", "0.000000"),
      Teleport::new("port", "456717.125", "211116.766", "-72903.164", "0.000000", "102.1200", "0.000000"),

      Teleport::new("northernmost", "-257550.594", "-555494.938", "-58510.559", "0.000000", "102.1200", "0.000000"),
      Teleport::new("northernmost", "-245547.922", "-563384.875", "-62342.133", "0.000000", "102.1200", "0.000000"),
    ];

    list
  }
}

/*
Murky:
(Lat: 326,736.5 Long: -97,513.727 Alt: -25,714.803)
(Lat: 305,660.906 Long: -122,063.469 Alt: -25,952.006)
(Lat: 299,523.375 Long: -108,436 Alt: -27,686.732)

Port:
(Lat: 400,536.188 Long: 269,163.625 Alt: -72,149.25)
(Lat: 400,997.656 Long: 255,636.422 Alt: -72,390.93)
(Lat: 411,201.594 Long: 243,380.125 Alt: -72,664.141)
(Lat: 456,717.125 Long: 211,116.766 Alt: -72,903.164)

Northwestern:
(Lat: -257,550.594 Long: -555,494.938 Alt: -58,510.559)
(Lat: -245,547.922 Long: -563,384.875 Alt: -62,342.133)

East swamp:
(Lat: 228,425.938 Long: 83,383.398 Alt: -74,882.508)
(Lat: 181,224.312 Long: 87,525.977 Alt: -74,756.484)

Sinkhole:
(Lat: -196,926.719 Long: -8,274.218 Alt: -71,082.219)
(Lat: -217,396.312 Long: -24,415.045 Alt: -62,561.25)

great falls1
{ 
  location_isle_v3: Some("X=-253624.859 Y=512152.906 Z=-25232.252"),
  rotation_isle_v3: Some("P=0.000000 Y=40.954926 R=0.000000"),
  camera_rotation_isle_v3: Some("P=0.000000 Y=130.954971 R=0.000000"),
  camera_distance_isle_v3: None
}
-253
512

Great falls 2
(Lat: -250,362.094 Long: 556,116.5 Alt: -20,790.934)


Wash1:
Isle location: Some("X=-378757.844 Y=301271.250 Z=-63352.480")
Isle Rotation: Some("P=0.000000 Y=102.122002 R=0.000000")
Camera rotation: Some("P=0.000000 Y=-167.877884 R=0.000000")
Camera distance: None

Wash2:
Isle location: Some("X=-370980.625 Y=341607.906 Z=-64949.922")
Isle Rotation: Some("P=0.000000 Y=-75.975044 R=0.000000")
Camera rotation: Some("P=0.000000 Y=14.025045 R=0.000000")
Camera distance: None

Landbridge:
(Lat: -351,908.312 Long: 67,331.461 Alt: -73,358.25)

North twins1:
(Lat: -463,301.094 Long: -202,647.391 Alt: -71,612.938)

(Lat: -397,790.938 Long: -200,770.906 Alt: -69,647.273)

Titan1:
(Lat: 61,883.891 Long: -160,841.297 Alt: -62,650.688)

(Lat: 22,028.447 Long: -169,786.203 Alt: -65,041.602)

(Lat: -62,766.828 Long: -239,913.812 Alt: -65,070.922)

(Lat: -63,213.469 Long: -302,744.25 Alt: -63,669.535)
*/