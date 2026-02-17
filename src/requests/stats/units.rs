use derive_more::{Add, Deref, DerefMut, From};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign};
use std::str::FromStr;
use thiserror::Error;

#[derive(Deref, DerefMut, From, Add, Copy, Clone)]
pub struct ThreeDecimalPlaceRateStat(f64);

impl ThreeDecimalPlaceRateStat {
	pub const NIL: Self = Self(f64::NAN);

	#[must_use]
	pub const fn new(inner: f64) -> Self {
		Self(inner)
	}
}

impl FromStr for ThreeDecimalPlaceRateStat {
	type Err = <f64 as FromStr>::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == ".---" {
			Ok(Self(f64::NAN))
		} else {
			Ok(Self(f64::from_str(s)?))
		}
	}
}

impl<'de> Deserialize<'de> for ThreeDecimalPlaceRateStat {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct StatVisitor;

		impl Visitor<'_> for StatVisitor {
			type Value = ThreeDecimalPlaceRateStat;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a float or string or .---")
			}

			fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
				Ok(ThreeDecimalPlaceRateStat::new(v))
			}

			fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
			where
				E: Error,
			{
				ThreeDecimalPlaceRateStat::from_str(&v).map_err(E::custom)
			}
		}

		deserializer.deserialize_any(StatVisitor)
	}
}

impl PartialEq for ThreeDecimalPlaceRateStat {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 || self.is_nan() && other.is_nan()
	}
}

impl Default for ThreeDecimalPlaceRateStat {
	fn default() -> Self {
		Self(f64::NAN)
	}
}

impl Display for ThreeDecimalPlaceRateStat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.0.is_nan() {
			write!(f, ".---")
		} else {
			write!(f, "{}", format!("{:.3}", self.0).trim_start_matches('0'))
		}
	}
}

impl Debug for ThreeDecimalPlaceRateStat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		<Self as Display>::fmt(self, f)
	}
}

#[derive(Deref, DerefMut, From, Add, PartialEq, Copy, Clone)]
pub struct PercentageStat(f64);

impl Eq for PercentageStat {}

impl PercentageStat {
	pub const NIL: Self = Self(f64::NAN);

	#[must_use]
	pub const fn new(inner: f64) -> Self {
		Self(inner)
	}
}

impl<'de> Deserialize<'de> for PercentageStat {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct PercentageStatVisitor;

		impl Visitor<'_> for PercentageStatVisitor {
			type Value = PercentageStat;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("Percentage")
			}

			fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
			where
				E: Error,
			{
				if !v.contains(|c: char| c.is_ascii_digit()) {
					Ok(PercentageStat::NIL)
				} else {
					Ok(PercentageStat::new(v.parse::<f64>().map_err(E::custom)? / 100.0))
				}
			}

			fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
				Ok(PercentageStat::new(v / 100.0))
			}

			#[allow(clippy::cast_lossless, reason = "needlessly pedantic")]
			fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
			where
				E: Error,
			{
				Ok(PercentageStat::new(v as f64 / 100.0))
			}
		}

		deserializer.deserialize_any(PercentageStatVisitor)
	}
}

impl Display for PercentageStat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.is_nan() {
			write!(f, "--.-%")
		} else {
			write!(f, "{:.2}%", self.0 * 100.0)
		}
	}
}

impl Debug for PercentageStat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.is_nan() {
			write!(f, "--.-%")
		} else {
			write!(f, "{}%", self.0 * 100.0)
		}
	}
}

impl Default for PercentageStat {
	fn default() -> Self {
		Self::NIL
	}
}

#[derive(Deref, DerefMut, From, Add, Copy, Clone)]
pub struct TwoDecimalPlaceRateStat(f64);

impl TwoDecimalPlaceRateStat {
	pub const NIL: Self = Self(f64::NAN);

	#[must_use]
	pub const fn new(inner: f64) -> Self {
		Self(inner)
	}
}

impl<'de> Deserialize<'de> for TwoDecimalPlaceRateStat {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct StatVisitor;

		impl Visitor<'_> for StatVisitor {
			type Value = TwoDecimalPlaceRateStat;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a float or string or -.--")
			}

			fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
				Ok(TwoDecimalPlaceRateStat::new(v))
			}

			fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
			where
				E: Error,
			{
				TwoDecimalPlaceRateStat::from_str(&v).map_err(E::custom)
			}
		}

		deserializer.deserialize_any(StatVisitor)
	}
}

impl FromStr for TwoDecimalPlaceRateStat {
	type Err = <f64 as FromStr>::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == "-.--" {
			Ok(Self(f64::NAN))
		} else {
			Ok(Self(f64::from_str(s)?))
		}
	}
}

impl PartialEq for TwoDecimalPlaceRateStat {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 || self.is_nan() && other.is_nan()
	}
}

impl Display for TwoDecimalPlaceRateStat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.is_nan() {
			write!(f, "-.--")
		} else {
			write!(f, "{:.2}", self.0)
		}
	}
}

impl Debug for TwoDecimalPlaceRateStat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		<Self as Display>::fmt(self, f)
	}
}

impl Default for TwoDecimalPlaceRateStat {
	fn default() -> Self {
		Self::NIL
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct InningsPitched {
	major: u32,
	minor: u8,
}

impl<'de> Deserialize<'de> for InningsPitched {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		String::deserialize(deserializer)?.parse::<Self>().map_err(Error::custom)
	}
}

impl Add for InningsPitched {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self::from_outs(self.as_outs() + rhs.as_outs())
	}
}

impl AddAssign for InningsPitched {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl InningsPitched {
	#[must_use]
	pub const fn from_outs(outs: u32) -> Self {
		Self {
			major: outs / 3,
			minor: (outs % 3) as u8,
		}
	}

	#[must_use]
	pub const fn new(whole_innings: u32, outs: u8) -> Self {
		Self { major: whole_innings, minor: outs }
	}

	#[must_use]
	pub fn as_fraction(self) -> f64 {
		self.into()
	}

	#[must_use]
	pub const fn as_outs(self) -> u32 {
		self.major * 3 + self.minor as u32
	}
}

impl From<InningsPitched> for f64 {

	#[allow(clippy::cast_lossless, reason = "needlessly pedantic")]
	fn from(value: InningsPitched) -> Self {
		value.major as Self + value.minor as Self / 3.0
	}
}

impl From<f64> for InningsPitched {
	#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "needlessly pedantic")]
	fn from(value: f64) -> Self {
		let value = value.max(0.0);
		let integer = value.trunc();
		let fractional = value - integer;
		let major = integer as u32;
		let minor = fractional as u8;
		Self { major, minor }
	}
}

impl Display for InningsPitched {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}", self.major, self.minor)
	}
}

#[derive(Debug, Error)]
pub enum InningsPitchedFromStrError {
	#[error("No . separator was present")]
	NoSeparator,
	#[error("Invalid whole inning quantity: {0}")]
	InvalidWholeInningsQuantity(String),
	#[error("Invalid inning out quantity: {0}")]
	InvalidOutsQuantity(String),
}

impl FromStr for InningsPitched {
	type Err = InningsPitchedFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (major, minor) = s.split_once('.').ok_or(InningsPitchedFromStrError::NoSeparator)?;
		let whole_innings = major.parse::<u32>().map_err(|_| InningsPitchedFromStrError::InvalidWholeInningsQuantity(major.to_owned()))?;
		let Ok(outs @ 0..3) = minor.parse::<u8>() else { return Err(InningsPitchedFromStrError::InvalidOutsQuantity(minor.to_owned())) };
		Ok(Self::new(whole_innings, outs))
	}
}

#[derive(Deref, DerefMut, From, Add, Copy, Clone)]
pub struct PlusStat(f64);

impl PlusStat {
	#[must_use]
	pub const fn new(x: f64) -> Self {
		Self(x)
	}
}

impl Display for PlusStat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.is_nan() {
			write!(f, "-")
		} else {
			write!(f, "{}", self.0.round() as i64)
		}
	}
}

/// Ex: Hits
pub type CountingStat = u32;

#[derive(Deserialize, PartialEq, Copy, Clone, Deref, DerefMut)]
pub struct FloatCountingStat<const N: usize>(f64);

impl<const N: usize> Add for FloatCountingStat<N> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl<const N: usize> AddAssign for FloatCountingStat<N> {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
	}
}

impl<const N: usize> Eq for FloatCountingStat<N> {

}

impl<const N: usize> FloatCountingStat<N> {
	#[must_use]
	pub const fn new(x: f64) -> Self {
		Self(x)
	}
}

impl<const N: usize> Display for FloatCountingStat<N> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.0.is_nan() {
			write!(f, "{:.N$}", "")
		} else {
			write!(f, "{:->N$}", self.0)
		}
	}
}

impl<const N: usize> Debug for FloatCountingStat<N> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		<Self as Display>::fmt(self, f)
	}
}
