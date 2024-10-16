Two traits `IOrderLookupRemover` and `IOrderSequentialRemover` implement a similar function.

```rs
    fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if self.pending_write() {
                self.group_position_remover()
                    .write_to_slot(ctx, outer_index);
            }
            self.outer_index_remover_mut().commit(ctx);
        }
    }
```

The structs are defined thus

```rs
pub trait IOrderLookupRemoverInner<'a> {
    /// Whether the bitmap group is pending a write
    fn pending_write(&self) -> bool;

    // Getters
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
    }

    /// To lookup and remove outer indices
    fn group_position_remover(&self) -> &impl IGroupPositionLookupRemover;


    /// To lookup and deactivate bits in bitmap groups
    fn outer_index_remover(&self) -> &impl IOuterIndexLookupRemover<'a>;

    /// Mutable reference to outer index remover
    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexLookupRemover<'a>;

    fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if self.pending_write() {
                self.group_position_remover()
                    .write_to_slot(ctx, outer_index);
            }
            self.outer_index_remover_mut().commit(ctx);
        }
    }
}

pub trait IOrderSequentialRemoverInner<'a> {
    /// Whether the bitmap group is pending a write
    fn pending_write(&self) -> bool;

    /// The current outer index
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
    }

    /// To lookup and remove outer indices
    fn group_position_remover(&self) -> &impl IGroupPositionSequentialRemover;

    /// To lookup and deactivate bits in bitmap groups
    fn outer_index_remover(&self) -> &impl IOuterIndexSequentialRemover<'a>;

    /// Mutable reference to outer index remover
    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexSequentialRemover<'a>;

    fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if self.pending_write() {
                self.group_position_remover()
                    .write_to_slot(ctx, outer_index);
            }
            self.outer_index_remover_mut().commit(ctx);
        }
    }
}
```

How to reduce redundant code? Most of the functions are same except for `IGroupPositionSequentialRemover`, `IGroupPositionLookupRemover`, `IOuterIndexSequentialRemover` and `IOuterIndexLookupRemover`
