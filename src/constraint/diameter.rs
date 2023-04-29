use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_DIAMETER},
    element::{AsHandle, TypeInfo},
    entity::{AsArc, Entity},
};

#[derive(Clone, Copy, Debug)]
pub struct Diameter<A: AsArc> {
    arc: Entity<A>,
    diameter: f64,
}

impl<A: AsArc> Diameter<A> {
    pub fn new(arc: Entity<A>, diameter: f64) -> Self {
        Self { arc, diameter }
    }
}

impl<A: AsArc> AsConstraintData for Diameter<A> {
    fn type_(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}

impl<A: AsArc> TypeInfo for Diameter<A> {
    fn type_of() -> String {
        format!("Diameter < {} >", A::type_of())
    }
}

impl<A: AsArc> From<Slvs_Constraint> for Diameter<A> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            arc: Entity::new(value.entityA),
            diameter: value.valA,
        }
    }
}
