use super::payloads::*;
use crate::core::{Block, BlockIndex, Bloxi, Chain, Node, Transaction};
use actix::prelude::*;
use futures::Future;
use std::collections::HashSet;

pub struct BloxiServerActor {
    id: Id,
    bloxi: Bloxi,
}

impl BloxiServerActor {
    pub fn new() -> BloxiServerActor {
        let id = Id::new();
        let bloxi = Bloxi::new();
        BloxiServerActor { id, bloxi }
    }
}

impl Actor for BloxiServerActor {
    type Context = Context<Self>;
}

// Messages
pub struct GetId;
simple_req_resp_impl!(GetId, Id);

impl Handler<GetId> for BloxiServerActor {
    type Result = Id;

    fn handle(&mut self, _: GetId, _: &mut Self::Context) -> Self::Result {
        self.id.clone()
    }
}

pub struct NewTransaction(pub Transaction);
simple_req_resp_impl!(NewTransaction, BlockIndex);

impl Handler<NewTransaction> for BloxiServerActor {
    type Result = BlockIndex;

    fn handle(
        &mut self,
        NewTransaction(transaction): NewTransaction,
        _: &mut Self::Context,
    ) -> Self::Result {
        let block_idx = self.bloxi.add_transaction(transaction);
        block_idx
    }
}

pub struct Mine;
simple_req_resp_impl!(Mine, Block);

impl Handler<Mine> for BloxiServerActor {
    type Result = Block;

    fn handle(&mut self, _: Mine, _: &mut Self::Context) -> Self::Result {
        self.bloxi.mine();
        self.bloxi.last_block().clone()
    }
}

pub struct GetChain;
simple_req_resp_impl!(GetChain, Chain);

impl Handler<GetChain> for BloxiServerActor {
    type Result = Chain;

    fn handle(&mut self, _: GetChain, _: &mut Self::Context) -> Self::Result {
        self.bloxi.chain()
    }
}

pub struct AddNode(pub Node);

#[derive(Serialize)]
pub struct CurrentNodes {
    nodes: HashSet<Node>,
}
simple_req_resp_impl!(AddNode, CurrentNodes);

impl Handler<AddNode> for BloxiServerActor {
    type Result = CurrentNodes;

    fn handle(&mut self, AddNode(node): AddNode, _: &mut Self::Context) -> Self::Result {
        let nodes = self.bloxi.register_node(node).clone();
        CurrentNodes { nodes }
    }
}

pub struct Reconcile;

impl Message for Reconcile {
    type Result = Result<Chain, ()>;
}

impl Handler<Reconcile> for BloxiServerActor {
    type Result = Box<Future<Item = Chain, Error = ()>>;

    fn handle(&mut self, _: Reconcile, context: &mut Self::Context) -> Self::Result {
        let self_addr = context.address().clone();
        let f = self
            .bloxi
            .reconcile()
            .and_then(move |reconciled| self_addr.send(UpdateBloxi(reconciled)).map_err(|_| ()));
        Box::new(f)
    }
}

pub struct UpdateBloxi(Bloxi);

impl Message for UpdateBloxi {
    type Result = Chain;
}

impl Handler<UpdateBloxi> for BloxiServerActor {
    type Result = Chain;

    fn handle(&mut self, UpdateBloxi(update): UpdateBloxi, _: &mut Self::Context) -> Self::Result {
        self.bloxi = update;
        self.bloxi.chain()
    }
}
