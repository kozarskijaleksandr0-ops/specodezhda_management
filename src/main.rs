mod contracts;
mod contracts_impl;
mod http;
mod models;
mod context;

use std::path::PathBuf;
use std::sync::Arc;

use async_sqlite::{Pool, PoolBuilder};
use ohkami::fang::Context;
use ohkami::{Ohkami, Route};
use context::RequestContext;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        let path_buf = PathBuf::from("database/data.db");
        let builder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);
        let pool: Pool = builder.open().await.expect("Database open error");

        let request_context = RequestContext::new(Arc::new(pool));

        let ceh_ohkami = Ohkami::new((
            "/add".POST(http::ceh::add_ceh),
            "/get/:id".GET(http::ceh::get_ceh),
            "/update/:id".PUT(http::ceh::update_ceh),
            "/remove/:id".DELETE(http::ceh::remove_ceh),
        ));

        let specodezhda_ohkami = Ohkami::new((
            "/add".POST(http::specodezhda::add_specodezhda),
            "/get/:id".GET(http::specodezhda::get_specodezhda),
            "/update/:id".PUT(http::specodezhda::update_specodezhda),
            "/remove/:id".DELETE(http::specodezhda::remove_specodezhda),
        ));

        let rabotnik_ohkami = Ohkami::new((
            "/add".POST(http::rabotnik::add_rabotnik),
            "/get/:id".GET(http::rabotnik::get_rabotnik),
            "/update/:id".PUT(http::rabotnik::update_rabotnik),
            "/remove/:id".DELETE(http::rabotnik::remove_rabotnik),
        ));

        let poluchenie_ohkami = Ohkami::new((
            "/add".POST(http::poluchenie::add_poluchenie),
            "/get".GET(http::poluchenie::get_poluchenie),
            "/remove".DELETE(http::poluchenie::remove_poluchenie),
        ));

        Ohkami::new((
            Context::new(request_context),
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
