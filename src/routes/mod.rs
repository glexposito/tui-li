use actix_web::web;

pub mod health;
pub mod redirect;
pub mod shorten;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/shorten").route(web::post().to(shorten::shorten_url)))
        .service(web::resource("/health").route(web::get().to(health::health)))
        .service(web::resource("/{id}").route(web::get().to(redirect::redirect_url)));
}
