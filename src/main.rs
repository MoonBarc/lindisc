use std::fs;

use actix_web::{web, get, HttpResponse, post, HttpRequest, HttpServer, App};
use linear::{LinearPayload, Data};
use reqwest::Client;

use crate::{discord::{Message, Embed, Footer, Field}, config::Config, linear::Action};

mod linear;
mod discord;
mod config;

const LINEAR_HOSTS: [&str; 2] = ["35.231.147.226", "35.243.134.228"];

#[post("/linput")]
async fn input(req: HttpRequest, http: web::Data<Client>, config: web::Data<Config>, payload: web::Json<LinearPayload>) -> HttpResponse {
    let ci = req.connection_info();
    let Some(ip) = ci.realip_remote_addr() else {
        println!("no ip found?");
        return HttpResponse::Unauthorized()
            .finish()
    };

    if !LINEAR_HOSTS.contains(&ip) {
        println!("imposter found! (@{})", ip);
        return HttpResponse::Unauthorized()
            .finish()
    }

    let discord_payload: Message;

    if Action::Remove == payload.action {
        return HttpResponse::Ok()
            .finish()
    }

    match &payload.data {
        Data::Issue { number, title, state, team, updated_at, .. } => {
            let color = usize::from_str_radix(&state.color.strip_prefix("#").unwrap(), 16).unwrap();
            discord_payload = Message {
                content: None,
                embeds: vec![
                    Embed {
                        title: format!("{}", title),
                        color,
                        description: "".to_string(),
                        footer: Footer {
                            text: format!("{}-{} • in {}", team.key, number, team.name),
                        },
                        timestamp: Some(updated_at.to_string()),
                        fields: vec![
                            Field {
                                name: "Status".to_string(),
                                value: state.name.to_string(),
                                inline: false
                            }
                        ]
                    }
                ]
            };
        }
    }

    let ok = http.post(config.webhook_url.clone())
        .json(&discord_payload)
        .header("Content-Type", "application/json")
        .send()
        .await;

    if let Err(e) = ok {
        eprintln!("Error during discord webhook send: {:#?}", e);
        return HttpResponse::InternalServerError()
            .finish()
    }

    HttpResponse::Ok()
        .finish()
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .body("ok!")
}

#[actix_web::main]
async fn main() {
    println!("Running lindisc v{}", env!("CARGO_PKG_VERSION"));

    let client = web::Data::new(Client::new());
    let config: web::Data<Config> = web::Data::new(toml::from_str(
        &fs::read_to_string("./config.toml").unwrap()
    ).unwrap());

    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(input)
            .app_data(client.clone())
            .app_data(config.clone())
    });

    server.bind("localhost:5252").unwrap()
        .run()
        .await
        .unwrap();
}
