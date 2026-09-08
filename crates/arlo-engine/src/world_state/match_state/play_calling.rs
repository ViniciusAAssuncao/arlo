use arlo_tactics::{PlayCall, PlayCallCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayCallTracker {
    home_script: Vec<PlayCall>,
    home_script_cursor: usize,
    away_script: Vec<PlayCall>,
    away_script_cursor: usize,
    home_manual_override: Option<PlayCall>,
    away_manual_override: Option<PlayCall>,
}

impl PlayCallTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn home_script(&self) -> &[PlayCall] {
        &self.home_script
    }

    pub fn away_script(&self) -> &[PlayCall] {
        &self.away_script
    }

    pub fn home_script_cursor(&self) -> usize {
        self.home_script_cursor
    }

    pub fn away_script_cursor(&self) -> usize {
        self.away_script_cursor
    }

    pub fn home_manual_override(&self) -> Option<&PlayCall> {
        self.home_manual_override.as_ref()
    }

    pub fn away_manual_override(&self) -> Option<&PlayCall> {
        self.away_manual_override.as_ref()
    }

    pub fn set_series_script(&mut self, is_home: bool, entries: Vec<PlayCall>) {
        if is_home {
            self.home_script = entries;
            self.home_script_cursor = 0;
        } else {
            self.away_script = entries;
            self.away_script_cursor = 0;
        }
    }

    pub fn set_manual_override(&mut self, is_home: bool, play_call: PlayCall) {
        if is_home {
            self.home_manual_override = Some(play_call);
        } else {
            self.away_manual_override = Some(play_call);
        }
    }

    pub fn resolve_and_consume(
        &mut self,
        is_home: bool,
        expected_category: PlayCallCategory,
    ) -> Option<PlayCall> {
        let manual_override = if is_home {
            self.home_manual_override.take()
        } else {
            self.away_manual_override.take()
        };

        if let Some(manual) = manual_override {
            return Some(manual);
        }

        let (script, cursor) = if is_home {
            (&self.home_script, &mut self.home_script_cursor)
        } else {
            (&self.away_script, &mut self.away_script_cursor)
        };

        if *cursor < script.len() {
            let candidate = &script[*cursor];
            if candidate.category() == expected_category {
                let chosen = candidate.clone();
                *cursor += 1;
                return Some(chosen);
            }
        }

        None
    }
}