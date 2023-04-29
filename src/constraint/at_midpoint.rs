use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, SLVS_C_AT_MIDPOINT},
    element::{AsHandle, TypeInfo},
    entity::{AsLineSegment, AsPoint, Entity, Workplane},
};

#[derive(Clone, Copy, Debug)]
pub struct AtMidpoint<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    point: Entity<P>,
    line: Entity<L>,
    workplane: Option<Entity<Workplane>>,
}

impl<P, L> AtMidpoint<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    pub fn new(point: Entity<P>, line: Entity<L>, workplane: Option<Entity<Workplane>>) -> Self {
        Self {
            point,
            line,
            workplane,
        }
    }
}

impl<P, L> AsConstraintData for AtMidpoint<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn type_(&self) -> i32 {
        SLVS_C_AT_MIDPOINT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.line.handle()])
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }
}

impl<P, L> TypeInfo for AtMidpoint<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn type_of() -> String {
        format!("AtMidpoint < {}, {} >", P::type_of(), L::type_of())
    }
}
impl<P, L> From<Slvs_Constraint> for AtMidpoint<P, L>
where
    P: AsPoint,
    L: AsLineSegment,
{
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            point: Entity::new(value.ptA),
            line: Entity::new(value.entityA),
            workplane: match value.wrkpl {
                0 => None,
                h => Some(Entity::new(h)),
            },
        }
    }
}
