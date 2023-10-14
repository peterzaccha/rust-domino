use super::domino::Domino;

pub enum PlayerMoveKind {
    PutToBack,
    PutToFront,
}

pub struct PlayerMove {
    idx: usize,
    kind: PlayerMoveKind,
}

impl PlayerMove {
    pub fn new(idx: usize, kind: PlayerMoveKind) -> Self {
        Self { idx, kind }
    }
}
