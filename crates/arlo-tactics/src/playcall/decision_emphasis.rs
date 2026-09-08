use arlo_domain::ArtrineDecisionKind;
use arlo_math::stats::UnipolarScalar;
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DecisionEmphasis {
    self_carry: UnipolarScalar,
    short_pass: UnipolarScalar,
    long_launch: UnipolarScalar,
    cross: UnipolarScalar,
    self_finish: UnipolarScalar,
}

impl DecisionEmphasis {
    pub fn new(
        self_carry: UnipolarScalar,
        short_pass: UnipolarScalar,
        long_launch: UnipolarScalar,
        cross: UnipolarScalar,
        self_finish: UnipolarScalar,
    ) -> Self {
        Self {
            self_carry,
            short_pass,
            long_launch,
            cross,
            self_finish,
        }
    }

    pub fn new_clamped(
        self_carry: f64,
        short_pass: f64,
        long_launch: f64,
        cross: f64,
        self_finish: f64,
    ) -> Self {
        Self {
            self_carry: UnipolarScalar::new_clamped(self_carry),
            short_pass: UnipolarScalar::new_clamped(short_pass),
            long_launch: UnipolarScalar::new_clamped(long_launch),
            cross: UnipolarScalar::new_clamped(cross),
            self_finish: UnipolarScalar::new_clamped(self_finish),
        }
    }

    pub fn self_carry(&self) -> UnipolarScalar {
        self.self_carry
    }

    pub fn short_pass(&self) -> UnipolarScalar {
        self.short_pass
    }

    pub fn long_launch(&self) -> UnipolarScalar {
        self.long_launch
    }

    pub fn cross(&self) -> UnipolarScalar {
        self.cross
    }

    pub fn self_finish(&self) -> UnipolarScalar {
        self.self_finish
    }

    pub fn get(&self, kind: ArtrineDecisionKind) -> UnipolarScalar {
        self[kind]
    }

    pub fn with_emphasis(mut self, kind: ArtrineDecisionKind, value: f64) -> Self {
        let scalar = UnipolarScalar::new_clamped(value);
        match kind {
            ArtrineDecisionKind::SelfCarry => self.self_carry = scalar,
            ArtrineDecisionKind::ShortPass => self.short_pass = scalar,
            ArtrineDecisionKind::LongLaunch => self.long_launch = scalar,
            ArtrineDecisionKind::Cross => self.cross = scalar,
            ArtrineDecisionKind::SelfFinish => self.self_finish = scalar,
        }
        self
    }
}

impl Index<ArtrineDecisionKind> for DecisionEmphasis {
    type Output = UnipolarScalar;

    fn index(&self, index: ArtrineDecisionKind) -> &Self::Output {
        match index {
            ArtrineDecisionKind::SelfCarry => &self.self_carry,
            ArtrineDecisionKind::ShortPass => &self.short_pass,
            ArtrineDecisionKind::LongLaunch => &self.long_launch,
            ArtrineDecisionKind::Cross => &self.cross,
            ArtrineDecisionKind::SelfFinish => &self.self_finish,
        }
    }
}