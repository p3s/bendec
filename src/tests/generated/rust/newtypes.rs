/** GENERATED BY BENDEC TYPE GENERATOR */
#[allow(unused_imports)]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// primitive built-in: u8

pub struct Public(pub u8);
impl std::ops::Deref for Public {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Foo, Bar)]
pub struct Generated(u8);

impl Generated {
  pub fn new(v: u8) -> Self {
    Self(v)
  }
}

impl std::ops::Deref for Generated {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct InCrate(pub(in crate::foo::bar) u8);

impl std::ops::Deref for InCrate {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct FooArray(pub [u8; 10]);

impl std::ops::Deref for FooArray {
  type Target = [u8; 10];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
