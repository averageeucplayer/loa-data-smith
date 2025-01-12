

#[macro_export]
macro_rules! create_struct_with_npcs {
    ($name:ident, $($field:ident => $key:expr),*) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name<'a> {
            $(pub $field: &'a crate::Npc),*
        }

        impl<'a> $name<'a> {
            pub fn from_npc_map(npc_map: &'a rustc_hash::FxHashMap<String, crate::Npc>) -> Self {
                Self {
                    $($field: npc_map.get($key).unwrap()),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! wrap_npc_struct {
    ($struct_name:ident, { $($field_name:ident: $field_type:ty),* $(,)? }) => {
        pub struct $struct_name<'a> {
            $(
                pub $field_name: $field_type,
            )*
        }

        impl<'a> $struct_name<'a> {
            pub fn from_npc_map(npc_map: &'a rustc_hash::FxHashMap<String, crate::Npc>) -> Self {
                Self {
                    $(
                        $field_name: <$field_type>::from_npc_map(npc_map),
                    )*
                }
            }
        }
    };
}