use rowan::{Checkpoint, Language};

use crate::syntax::{RenLang, SyntaxPart};

use super::Parser;

pub(crate) struct Marker {
    checkpoint: rowan::Checkpoint,
    label: &'static str,
    bomb: drop_bomb::DropBomb,
}
impl Marker {
    pub(super) fn new(checkpoint: Checkpoint, label: &'static str) -> Self {
        Self {
            checkpoint,
            label,
            bomb: drop_bomb::DropBomb::new(format!("Incomplete marker {}", label)),
        }
    }
    pub fn complete<K: Into<SyntaxPart>, E>(mut self, p: &mut Parser<E>, kind: K) {
        self.bomb.defuse();
        p.builder
            .start_node_at(self.checkpoint, RenLang::kind_to_raw(kind.into()));
        p.builder.finish_node();
    }
    pub fn discard(mut self) {
        self.bomb.defuse();
    }
    pub fn commit<K: Into<SyntaxPart>, E>(&mut self, p: &mut Parser<E>, kind: K) {
        std::mem::replace(self, Marker::new(self.checkpoint, self.label)).complete(p, kind);
    }
}
