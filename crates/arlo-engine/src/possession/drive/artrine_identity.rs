use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrueArtrine(Uuid);

impl TrueArtrine {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn id(&self) -> Uuid {
        self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TrueArtrine {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FalseArtrine(Uuid);

impl FalseArtrine {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn id(&self) -> Uuid {
        self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for FalseArtrine {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtrineCarrier {
    True(TrueArtrine),
    False(FalseArtrine),
}

impl ArtrineCarrier {
    pub fn id(&self) -> Uuid {
        match self {
            Self::True(ta) => ta.id(),
            Self::False(fa) => fa.id(),
        }
    }

    pub fn is_true(&self) -> bool {
        matches!(self, Self::True(_))
    }

    pub fn is_false(&self) -> bool {
        matches!(self, Self::False(_))
    }

    pub fn as_true(&self) -> Option<TrueArtrine> {
        match self {
            Self::True(ta) => Some(*ta),
            Self::False(_) => None,
        }
    }

    pub fn as_false(&self) -> Option<FalseArtrine> {
        match self {
            Self::True(_) => None,
            Self::False(fa) => Some(*fa),
        }
    }
}

impl From<TrueArtrine> for ArtrineCarrier {
    fn from(ta: TrueArtrine) -> Self {
        Self::True(ta)
    }
}

impl From<FalseArtrine> for ArtrineCarrier {
    fn from(fa: FalseArtrine) -> Self {
        Self::False(fa)
    }
}
