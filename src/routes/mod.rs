use actix_web::web;

pub mod shorten;
pub mod redirect;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/shorten").route(web::post().to(shorten::shorten_url)))
       .service(web::resource("/{id}").route(web::get().to(redirect::redirect_url)));
}
