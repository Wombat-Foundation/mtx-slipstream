use crate::MilliSecondsSinceUnixEpoch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Medium {
	Email,
	Msisdn,
}
crate::impl_codec_enum!(Medium { Email => "email", Msisdn => "msisdn" });

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThirdPartyIdentifier {
	pub address: String,
	pub medium: Medium,
	pub validated_at: MilliSecondsSinceUnixEpoch,
	pub added_at: MilliSecondsSinceUnixEpoch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThirdPartyIdentifierInit {
	pub address: String,
	pub medium: Medium,
	pub validated_at: MilliSecondsSinceUnixEpoch,
	pub added_at: MilliSecondsSinceUnixEpoch,
}

impl crate::codec::Serialize for ThirdPartyIdentifier {
	fn to_json(&self) -> crate::json::Value {
		crate::json!({"address": self.address, "medium": self.medium, "validated_at": self.validated_at, "added_at": self.added_at})
	}
}
impl crate::codec::Deserialize for ThirdPartyIdentifier {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		let object =
			value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
		let field = |name| object.get(name).ok_or_else(|| crate::codec::DeError::expected(name));
		Ok(Self {
			address: String::from_json(field("address")?)?,
			medium: Medium::from_json(field("medium")?)?,
			validated_at: MilliSecondsSinceUnixEpoch::from_json(field("validated_at")?)?,
			added_at: MilliSecondsSinceUnixEpoch::from_json(field("added_at")?)?,
		})
	}
}

impl From<ThirdPartyIdentifierInit> for ThirdPartyIdentifier {
	fn from(value: ThirdPartyIdentifierInit) -> Self {
		Self {
			address: value.address,
			medium: value.medium,
			validated_at: value.validated_at,
			added_at: value.added_at,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Medium, ThirdPartyIdentifier};
	use crate::{
		MilliSecondsSinceUnixEpoch,
		codec::{from_str, to_string},
	};

	#[test]
	fn third_party_identifier_literal_json_golden() {
		let json =
			r#"{"added_at":42,"address":"a@example.org","medium":"email","validated_at":7}"#;
		let value: ThirdPartyIdentifier = from_str(json).expect("valid third-party identifier");
		assert_eq!(value.medium, Medium::Email);
		assert_eq!(value.validated_at, MilliSecondsSinceUnixEpoch(7));
		assert_eq!(to_string(&value), json);
	}

	#[test]
	fn medium_literal_json_golden() {
		assert_eq!(to_string(&Medium::Msisdn), "\"msisdn\"");
		assert_eq!(from_str::<Medium>("\"email\"").unwrap(), Medium::Email);
	}
}
