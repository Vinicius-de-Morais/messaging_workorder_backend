use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use setup::set_up_db;


#[macro_use] extern crate rocket;

pub mod setup;
pub mod entities;

#[get("/")]
async fn index(db: &State<DatabaseConnection>) -> String {
    save_user(db).await.to_string()
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
      Ok(db) => db,
      Err(err) => panic!("{}", err),
  };

    rocket::build()
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            ..rocket::Config::default()
        })
        .manage(db)
        .mount("/", routes![index])

    
}

async fn save_user(db: &DatabaseConnection) -> Uuid {
    let new_user = entities::user::ActiveModel {
        id: Set(Uuid::new_v4()),
        ..Default::default()
    };

    let user = entities::user::Entity::insert(new_user)
        .exec(db)
        .await
        .unwrap();
    
    return user.last_insert_id;
}