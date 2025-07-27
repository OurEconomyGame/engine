use super::Material;
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Recipe<'a> {
    pub inputs: Cow<'a, [(Material, u32)]>,
}

impl<'a> Recipe<'a> {
    pub const fn food() -> Recipe<'static> {
        Recipe {
            inputs: Cow::Borrowed(&[
                (Material::Electricity, 10),
                (Material::Water, 5),
                (Material::Grain, 5),
            ]),
        }
    }

    pub const fn empty() -> Recipe<'static> {
        Recipe {
            inputs: Cow::Borrowed(&[]),
        }
    }

    pub fn dynamic(inputs: Vec<(Material, u32)>) -> Recipe<'static> {
        Recipe {
            inputs: Cow::Owned(inputs),
        }
    }
}

impl fmt::Display for Recipe<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Recipe:")?;
        for (mat, amount) in self.inputs.iter() {
            writeln!(f, "- {} {}", amount, mat.unit())?;
        }
        Ok(())
    }
}
