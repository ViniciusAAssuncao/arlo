use crate::units::duration::Duration;
use crate::units::length::Length;
use crate::units::scalar_macro::define_rate_of_change;
use crate::units::speed::Speed;

define_rate_of_change!(Speed, Duration, Length);
