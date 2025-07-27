#[macro_export]
macro_rules! define_materials {
    ($( $mat:ident => ($field:ident, $unit:expr) ),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Material {
            $( $mat ),*
        }

        impl Material {
            pub fn unit(&self) -> &'static str {
                match self {
                    $(Material::$mat => $unit),*
                }
            }

            pub fn display_name(&self) -> &'static str {
                match self {
                    $(Material::$mat => stringify!($mat)),*
                }
            }

            pub fn from_str(name: &str) -> Option<Material> {
                match name {
                    $(stringify!($mat) => Some(Material::$mat)),*,
                    _ => None,
                }
            }

            pub fn to_string_key(&self) -> &'static str {
                self.display_name()
            }

            pub fn all() -> &'static [Material] {
                &[$(Material::$mat),*]
            }
        }

        impl std::fmt::Display for Material {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} ({})", self.display_name(), self.unit())
            }
        }

        #[derive(Debug, Clone, Copy, Default)]
        pub struct Inventory {
            $(pub $field: u32),*
        }

        impl Inventory {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn add(&mut self, mat: Material, amount: u32) {
                match mat {
                    $(Material::$mat => self.$field += amount),*
                }
            }
        }
    };
}
