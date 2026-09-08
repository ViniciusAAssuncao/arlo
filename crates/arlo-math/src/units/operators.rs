use crate::units::duration::Duration;
use crate::units::length::Length;
use crate::units::position::Position;
use crate::units::scalar_macro::define_rate_of_change;
use crate::units::speed::Speed;
use crate::units::velocity::Velocity;

define_rate_of_change!(Speed, Duration, Length);

impl std::ops::Mul<Duration> for Velocity {
    type Output = Position;
    fn mul(self, rhs: Duration) -> Self::Output {
        Position::from_raw(self.raw() * rhs.value())
    }
}
