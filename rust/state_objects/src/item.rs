//! The item module contains the `Item` struct, which represents a single item in storage.

use ixc_core::{Context, Result};
use ixc_schema::state_object::ObjectValue;
use crate::Map;

/// A single item in storage.
pub struct Item<V> {
    map: Map<(), V>,
}

impl<V: ObjectValue> Item<V>
where
        for<'a> V::Out<'a>: Default,
{
    /// Gets the value of the item.
    pub fn get<'value>(&self, ctx: &'value Context) -> Result<V::Out<'value>> {
        let v = self.map.get(ctx, &())?;
        Ok(v.unwrap_or_default())
    }

    /// Sets the value of the item.
    pub fn set(&self, ctx: &mut Context, value: V::In<'_>) -> Result<()> {
        self.map.set(ctx, &(), &value)
    }
}
