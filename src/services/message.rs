use serenity::{model::{channel::{Message, ReactionType}, prelude::User}, prelude::*, utils::Colour};

use crate::models::{dino::Dino, garage::Garage, teleport::Teleport};

pub struct MessageResponder<'a> {
  pub ctx: &'a Context,
  pub msg: &'a Message,
}

impl MessageResponder<'_> {
  pub async fn in_progress<D>(
    &self,
    title: D,
    message: D,
  ) -> Result<Message, serenity::Error> where D: ToString  {
    let result = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.colour(Colour::from_rgb(200, 200, 200));
          e
      });
      m.reference_message(self.msg);
      m
    }).await;

    result
  }

  pub async fn error<D>(
    &self,
    title: D,
    message: D,
  ) where D: ToString  {
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.colour(Colour::from_rgb(200, 60, 20));
          e
      });
      m.reference_message(self.msg);
      m
    }).await;
  }

  pub async fn success_norply<D>(
    &self,
    title: D,
    message: D,
  ) where D: ToString  {
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.colour(Colour::from_rgb(50, 220, 50));
          e
      });
      m
    }).await;
  }

  pub async fn success<D>(
    &self,
    title: D,
    message: D,
  ) where D: ToString  {
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.colour(Colour::from_rgb(50, 220, 50));
          e
      });
      m.reference_message(self.msg);
      m
    }).await;
  }

  pub async fn dino_garage(
    &self,
    slots: &Vec<Garage>,
    is_delete: bool
  ) -> Result<Message, serenity::Error> {
    let mut list_str = String::new();
    let mut count: i32 = 1;
    let mut reactions = Vec::new();

    let emojis = vec![
      "❌",
      "1️⃣",
      "2️⃣",
      "3️⃣",
      "4️⃣",
      "5️⃣"
    ];
    reactions.push(ReactionType::Unicode("❌".to_string()));
    for slot in slots {
      let dino_name = Dino::game_identifier_to_display_name(&slot.character_class);
      list_str.push_str(&format!("{}) {}\n", count, dino_name));
      if let Some(emoji) = emojis.get(count as usize) {
        let reaction = ReactionType::Unicode(emoji.to_string());
        reactions.push(reaction);
        count += 1;
      }
    }
    let type_format: String;
    if is_delete {
      type_format = "delete".to_string();
    } else {
      type_format = "release".to_string();
    }
    list_str.push_str(&format!("\nSelect which dino you would like to {}:", type_format));

    let result = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title("Your dino garage:");
          e.description(list_str);
          e.colour(Colour::from_rgb(50, 220, 50));
          e
      });
      m.reactions(reactions);
      m.reference_message(self.msg);
      m
    }).await;

    result
  }

  pub async fn dino_garage_swap(
    &self,
    slots: &Vec<Garage>,
    other_dino: &String,
  ) -> Result<Message, serenity::Error> {
    let mut list_str = String::new();
    let mut count: i32 = 1;
    let mut reactions = Vec::new();

    let emojis = vec![
      "❌",
      "1️⃣",
      "2️⃣",
      "3️⃣",
      "4️⃣",
      "5️⃣"
    ];
    reactions.push(ReactionType::Unicode("❌".to_string()));
    for slot in slots {
      let dino_name = Dino::game_identifier_to_display_name(&slot.character_class);
      list_str.push_str(&format!("{}) {}\n", count, dino_name));
      if let Some(emoji) = emojis.get(count as usize) {
        let reaction = ReactionType::Unicode(emoji.to_string());
        reactions.push(reaction);
        count += 1;
      }
    }

    list_str.push_str(&format!("\nSelect which dino you would like to swap your **{}** with:", other_dino));

    let result = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title("Swap dino:");
          e.description(list_str);
          e.colour(Colour::from_rgb(50, 220, 50));
          e
      });
      m.reactions(reactions);
      m.reference_message(self.msg);
      m
    }).await;

    result
  }

  pub async fn cb_usage(&self) {
    let dino_list = Dino::list();
    let list = dino_list.iter().filter(|&d| d.enabled == true).collect::<Vec<&Dino>>();
    let field_list = list.iter().map(|&d| {
      (d.display_name.to_string(), d.aliases[0].to_string(), true)
    });
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title("Usage gg.cb");
          e.description("gg.cb dino male|female|m|f");
          e.author(|a| {
              a.name(&self.msg.author.name);
              a.icon_url(self.msg.author.avatar_url().unwrap());
              a
          });
          e.fields(field_list);
          e.colour(Colour::from_rgb(50, 100, 230));
          e.footer(|f| {
              f.text("Example: gg.cb trex fem");
              f
          });
          e
      });
      m.reference_message(self.msg);
      m
    }).await;
  }

  pub async fn respond_injection<D>(
    &self,
    title: D,
    message: D,
    cash: i64,
    bank: i64,
    cost: i64
  ) where D: ToString {
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.author(|a| {
              a.name(&self.msg.author.name);
              a.icon_url(self.msg.author.avatar_url().unwrap());
              a
          });
          e.fields(vec![
              ("Cash", format!("{}", cash), true),
              ("Bank", format!("{}", bank), true),
          ]);
          e.colour(Colour::from_rgb(50, 220, 50));
          e.footer(|f| {
              f.text(format!("{} Points were withdrawn from your cash", cost));
              f
          });
          e
      });
      m.reference_message(self.msg);
      m
    }).await;
  }

  pub async fn respond_admin_injection<D>(
    &self,
    title: D,
    message: D,
    user: &User
  ) where D: ToString {
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.author(|a| {
              a.name(&user.name);
              a.icon_url(user.avatar_url().unwrap());
              a
          });
          e.colour(Colour::from_rgb(50, 220, 50));
          e.footer(|f| {
              f.text("Admin injections do not cost points");
              f
          });
          e
      });
      m.reference_message(self.msg);
      m
    }).await;
  }

  pub async fn respond_tp<D>(
    &self,
    title: D,
    message: D,
    cash: i64,
    bank: i64,
    cost: i64,
    user: &User
  ) where D: ToString {
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.author(|a| {
              a.name(&user.name);
              a.icon_url(user.avatar_url().unwrap());
              a
          });
          e.fields(vec![
            ("Cash", format!("{}", cash), true),
            ("Bank", format!("{}", bank), true),
          ]);
          e.colour(Colour::from_rgb(50, 220, 50));
          e.footer(|f| {
              f.text(format!("{} points withdrawn for the teleport", cost));
              f
          });
          e
      });
      m.reference_message(self.msg);
      m
    }).await;
  }

  pub async fn tp_usage(
    &self,
  ) {
    let list = Teleport::tp_locations();
    let mut known: Vec<String> = vec![];
    let mut locations = String::new();
    for location in list {
      if known.iter().any(|i| i.contains(&location.label) == true) {
        continue;
      }
      let label = location.label.to_string();
      locations.push_str(&format!("{}/", &label));
      known.insert(0, label);
    }
    let mut final_location_list = locations.chars();
    final_location_list.next_back();
    let location_list_str = final_location_list.as_str();
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title("TP command usage");
          e.description(&format!("gg.tp {}", location_list_str));
          e.colour(Colour::from_rgb(50, 50, 220));
          e.footer(|f| {
            f.text("A tp costs 1000 points");
            f
          });
          e
      });
      m.reference_message(self.msg);
      m
    }).await;
  }

  pub async fn injection_usage<D>(
    &self,
    title: D,
    message: D,
    cash: i64,
    bank: i64,
    cost: i64
  ) where D: ToString {
    let _ = self.msg.channel_id.send_message(&self.ctx.http, |m| {
      m.embed(|e| {
          e.title(title);
          e.description(message);
          e.author(|a| {
              a.name(&self.msg.author.name);
              a.icon_url(self.msg.author.avatar_url().unwrap());
              a
          });
          e.fields(vec![
              ("Cash", format!("{}", cash), true),
              ("Bank", format!("{}", bank), true),
          ]);
          e.colour(Colour::from_rgb(0, 100, 200));
          e.footer(|f| {
              f.text(format!("{} Points were withdrawn from your cash", cost));
              f
          });
          e
      });
      m
    }).await;
  }
}

// m.reactions(reactions.into_iter());
        // m.embed(|e| {
        //     e.title("Dino injected");
        //     e.description("This is a description");
        //     e.author(|a| {
        //         a.name(&msg.author.name);
        //         a.icon_url(msg.author.avatar_url().unwrap());

        //         a
        //     });
        //     e.colour(Colour::from_rgb(0, 255, 0));
        //     // e.image("attachment://ferris_eyes.png");
        //     e.fields(vec![
        //         ("This is the first field", "This is a field body", true),
        //         ("This is the second field", "Both of these fields are inline", true),
        //     ]);
        //     e.field("This is the third field", "This is not an inline field", false);
        //     e.footer(|f| {
        //         f.text("This is a footer");

        //         f
        //     });

        //     e
        // });
        // m.add_file(AttachmentType::Path(Path::new("./ferris_eyes.png")));
