use rowan::{Checkpoint, Language};

use crate::syntax::{SyntaxPart, RenLang};

use super::Parser;

pub(crate) struct Marker {
    checkpoint: rowan::Checkpoint,
    bomb: drop_bomb::DropBomb,
}
impl Marker {
    pub fn new(checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint,
            bomb: drop_bomb::DropBomb::new("Markers need to be completed"),
        }
    }
    pub fn complete<K: Into<SyntaxPart>>(mut self, p: &mut Parser, kind: K) {
        self.bomb.defuse();
        p.builder.start_node_at(self.checkpoint, RenLang::kind_to_raw(kind.into()));
        p.builder.finish_node();
    }
    pub fn discard(mut self) {
        self.bomb.defuse();
    }
    pub fn commit<K: Into<SyntaxPart>>(&mut self, p: &mut Parser, kind: K) {
        std::mem::replace(self, Marker::new(self.checkpoint)).complete(p, kind);
    }
}
