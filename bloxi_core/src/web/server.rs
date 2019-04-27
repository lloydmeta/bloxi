use actix::prelude::*;

use super::actors::*;
use super::handlers::*;

use std::io;

use actix_web::{http, server, App};

pub struct Server {
    port: usize,
    actor: Addr<BloxiServerActor>,
}

impl Server {
    pub fn new(port: usize) -> Server {
        let actor = BloxiServerActor::new().start();
        Server { port, actor }
    }

    pub fn run(self) -> Result<i32, io::Error> {
        let actor = self.actor;
        let run_on = format!("127.0.0.1:{}", self.port);

        let sys = System::new("bloxi-server");
        // routes
        server::new(move || {
            let get_id_handler = GetIdHandler(actor.clone());
            let new_tx_handler = NewTransactionHandler(actor.clone());
            let mine_handler = MineHandler(actor.clone());
            let get_chain_handler = GetChainHandler(actor.clone());
            let add_node_handler = AddNodeHandler(actor.clone());
            let reconcile_handler = ReconcileHandler(actor.clone());
            App::new()
                .resource("/id", move |r| {
                    r.method(http::Method::GET).h(get_id_handler)
                })
                .resource("/chain", move |r| {
                    r.method(http::Method::GET).h(get_chain_handler)
                })
                .resource("/transaction", move |r| {
                    r.method(http::Method::POST).h(new_tx_handler)
                })
                .resource("/mine", move |r| {
                    r.method(http::Method::POST).h(mine_handler)
                })
                .resource("/node", move |r| {
                    r.method(http::Method::POST).h(add_node_handler)
                })
                .resource("/reconcile", move |r| {
                    r.method(http::Method::POST).h(reconcile_handler)
                })
        })
        .bind(&run_on)?
        .start();

        info!("Running at: {}", run_on);
        Ok(sys.run())
    }
}
