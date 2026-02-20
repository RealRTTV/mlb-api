id!(#[doc = "A [`String`] representing a Game Logic Event,\nlike a count change, new batter, game status change, etc."] LogicalEventId { code: String });

meta_kind_impl!("logicalEvents" => LogicalEventId);
test_impl!(LogicalEventId);
