mod contracts;
mod contracts_impl;
mod http;
mod models;

use ohkami::{Ohkami, Route};

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        // Цех
        let ceh_ohkami = Ohkami::new((
            "/add".POST(http::ceh::add_ceh),
            "/get".GET(http::ceh::get_ceh),
            "/update".PUT(http::ceh::update_ceh),
            "/remove".DELETE(http::ceh::remove_ceh),
        ));

        // Спецодежда
        let specodezhda_ohkami = Ohkami::new((
            "/add".POST(http::specodezhda::add_specodezhda),
            "/get".GET(http::specodezhda::get_specodezhda),
            "/update".PUT(http::specodezhda::update_specodezhda),
            "/remove".DELETE(http::specodezhda::remove_specodezhda),
        ));

        // Работник
        let rabotnik_ohkami = Ohkami::new((
            "/add".POST(http::rabotnik::add_rabotnik),
            "/get".GET(http::rabotnik::get_rabotnik),
            "/update".PUT(http::rabotnik::update_rabotnik),
            "/remove".DELETE(http::rabotnik::remove_rabotnik),
        ));

        // Получение
        let poluchenie_ohkami = Ohkami::new((
            "/add".POST(http::poluchenie::add_poluchenie),
            "/get".GET(http::poluchenie::get_poluchenie),
            "/update".PUT(http::poluchenie::update_poluchenie),
            "/remove".DELETE(http::poluchenie::remove_poluchenie),
        ));
        Ohkami::new((
            "/ceh".By(ceh_ohkami),
            "/specodezhda".By(specodezhda_ohkami),
            "/rabotnik".By(rabotnik_ohkami),
            "/poluchenie".By(poluchenie_ohkami),
        ))
        .howl("localhost:3000")
        .await;

        Ok(())
    })
}
