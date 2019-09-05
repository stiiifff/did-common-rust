#[macro_export]
macro_rules! did {
	($did: expr) => {
		//TODO: If cannot parse, should generate error w/ parsing error
		Did::parse($did).unwrap()
	};
}
