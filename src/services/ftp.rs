use async_ftp::FtpStream;
use async_ftp::FtpError;
use async_trait::async_trait;
use bb8::ManageConnection;
use std::ops::DerefMut;

pub struct FtpConnectionManager {
  pub address: String,
  pub username: String,
  pub password: String,
}

impl FtpConnectionManager {
  pub fn new(to_address: &String, username: &String, password: &String) -> FtpConnectionManager {
    FtpConnectionManager {
      address: to_address.to_string(),
      username: username.to_string(),
      password: password.to_string()
    }
  }
}

#[async_trait]
impl ManageConnection for FtpConnectionManager {
  type Connection = FtpStream;
  type Error = FtpError;

  async fn connect(&self) -> Result<Self::Connection, Self::Error> {
    let mut ftp_stream = match FtpStream::connect(self.address.to_string()).await {
      Ok(stream) => stream,
      Err(err) => {
        return Err(err);
      }
    };

    println!("=> FTP Connected, authenticating...");
    match ftp_stream.login(&self.username, &self.password).await {
      Err(err) => {
        println!("FTP Error: {:?}", err);
        return Err(err);
      },
      _ => (),
    };
    println!("=> FTP Authenticated, locating player files");
    match ftp_stream.cwd("104.206.108.250_14020/TheIsle/Saved/Databases/Survival/Players").await {
      Err(err) => {
        return Err(err);
      },
      _ => (),
    };
    println!("=> Player files located");

    return Ok(ftp_stream);
  }

  async fn is_valid(
    &self,
    conn: &mut bb8::PooledConnection<'_, Self>
  ) -> Result<(), FtpError> {
    println!("=> Checking connection health");
    let ftp_stream = conn.deref_mut();

    match ftp_stream.pwd().await {
      Ok(_) => {
        println!("=> Connection: [ OK ]");
        return Ok(());
      },
      Err(err) => {
        println!("=> Connection: [ ERR ]");
        return Err(err);
      }
    }
  }

  fn has_broken(&self, _client: &mut Self::Connection) -> bool {
    false
  }
}
