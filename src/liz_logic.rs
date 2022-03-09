use rlua::UserData;

pub fn sense_same() -> Sense {
    Sense::Same
}

pub fn sense_swap() -> Sense {
    Sense::Swap
}

pub fn sense_apply(sense: Sense, apply_to: bool) -> bool {
    if sense == Sense::Same {
        apply_to
    } else {
        !apply_to
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sense {
    Same,
    Swap,
}

impl UserData for Sense {}
