
#[macro_export]
macro_rules! msg_impl {
	($msg_struct:ident: $type:ty) => {
		impl Message for $msg_struct {
			type Result = $type;
		}
	};
}

#[macro_export]
macro_rules! msg {
	($(
		$(#[$outer:meta])* 
		$msg_name:ident ($(
			$fields:ty $(,)?
		) *) -> $type:ty $(,)?) *) => {
		$(
			#[derive(Debug)]
			$(#[$outer])*
			pub struct $msg_name($(pub $fields, )*);
        	crate::msg_impl!($msg_name: $type);	
		)*
	};
	($(
		$(#[$outer:meta])* 
		$msg_name:ident { 
			$(
				$(#[$inner:ident $($args:tt)*])*
				$field_name:ident: $fields:ty $(,)?
			)* 
		} -> $type:ty$(,)?) *) => {
		$(
			#[derive(Debug)]
			$(#[$outer])*
			pub struct $msg_name {
				$(
					$(#[$inner $($args)*])*
					pub $field_name: $fields,
				)*
        	}
        crate::msg_impl!($msg_name: $type);)*
	};
    () => {}
}
