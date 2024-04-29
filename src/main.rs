mod post;
mod notification;
mod parser;
mod deduplicator;



use notification::{send_notification, NotificationChannel};

#[macro_use]
extern crate rocket;


use rocket::{Request, State, tokio};

use std::{sync::Arc, time::Duration, env};
use rocket::tokio::sync::MutexGuard;

use tokio::sync::Mutex;
// use crate::deduplicator;

// Define a shared state for the `my_config` variable
struct AppState {
    my_config: Arc<Mutex<String>>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/config", data = "<config>")]
async fn set_config(config: String, my_state: &State<AppState>) -> &'static str {
    println!("{}", config);
    // Get Mutable access to shared data
    let mut my_shared_config = my_state.my_config.lock().await;
    // *my_shared_config = "Updated value from API".to_string();
    *my_shared_config = config;
    // my_shared_config.clone()
    "Config updated successfully"
}

// #[post("/post", data = "<body>")]
// fn post_handler(body: Json<RequestBody>) -> String {
//     let request_body = body.into_inner();
//     let message = request_body.message;
//
//     // Do something with the message
//     format!("Received message: {}", message)
// }

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

// #[launch]
// fn rocket() -> _ {
//     // Spawn a new task to run the periodic function
//     tokio::spawn(periodic_task());
//     rocket::build()
//         .mount("/", routes![index, hello])
//         .register("/", catchers![not_found])
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // an Arc<Mutex<String>> to hold the shared data. Arc allows us to safely share the data across threads, and Mutex provides mutual exclusion to prevent data races.
    // let my_shared_config = Arc::new(Mutex::new("Initial value".to_string()));
    let my_config = Arc::new(Mutex::new(String::new()));
    let my_state = AppState { my_config: my_config.clone() };

    // Spawn a new task to run the periodic function
    // Spawn a separate thread for the periodic task
    let my_config_clone = my_config.clone();
    //let my_shared_config_clone = my_shared_config.clone();
    tokio::spawn(periodic_task(my_config_clone));

    let _ = rocket::build()
        .mount("/", routes![index, hello, set_config])
        .register("/", catchers![not_found])
        .manage(my_state)
        .launch()
        .await?;

    Ok(())
}
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, {} was not found.", req.uri())
}

async fn periodic_task(my_config: Arc<Mutex<String>>) {

    let mut interval = tokio::time::interval(Duration::from_secs(5)); // 5 minutes

    loop {
        interval.tick().await;
        println!("Invoking periodic function...");

        // // Get Immutable access to shared data
        // let my_shared_config = my_config.lock().await;
        
        // println!("{}", my_config);

        let slick_popular_deduplicator = deduplicator::Deduplicator::new(500);
        // let mut slick_all_deduplicator = deduplicator::Deduplicator::new(500);
        // Call your periodic function here
        process_slick_popular(&my_config, slick_popular_deduplicator).await;
        // process_slick_all(my_shared_config, slick_all_deduplicator).await;
    }
}

async fn process_slick_popular(my_config: &Arc<Mutex<String>>, deduplicator: deduplicator::Deduplicator) {
    let url = env::var("MY_URL").unwrap_or_else(|e| {
        eprintln!("Env MY_URL not set: {}", e);
        String::new()
    });


    // let output_dir = std::env::args().nth(2).expect("no dir provided");
    let notification_channel = parser::NotificationChannel::Telegram;

    let mut op = parser::Parser::new(url, notification_channel);
    op.fetch_and_parse()
        .await
        .expect("Could not fetch & parse posts");

    // Get Immutable access to shared data
    let my_shared_config = my_config.lock().await;
    // println!("{}", my_shared_config);


    let filtered_posts = op.filter_posts(&my_shared_config, deduplicator);

    let telegram_channel = NotificationChannel::Telegram("YOUR_BOT_TOKEN".to_string());
    

    // loop over filtered_post and call send_notification for each post
    for post in filtered_posts {
        send_notification(&post, &telegram_channel).unwrap();
    }
    // let save_op = match op.save_dir_exists() {
    //     true => {
    //         let overwrite = prompt_default(
    //             format!(
    //                 "directory {} exists. Do you want to overwrite it?",
    //                 &op.output_dir
    //             ),
    //             false,
    //         )
    //             .unwrap();
    //         op.save_files(overwrite)
    //     }
    //     false => op.save_files(false),
    // };

    // match save_op {
    //     Ok(status) => println!("Completed: {}", status),
    //     Err(e) => return Err(format!("{}", e)),
    // }

    println!("Periodic function executed!");
    // Ok(())
}