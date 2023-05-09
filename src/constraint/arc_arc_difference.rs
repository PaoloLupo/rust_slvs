use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_ARC_DIFFERENCE},
    element::AsHandle,
    entity::{ArcOfCircle, EntityHandle},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcArcDifference {
    pub group: Group,
    pub arc_a: EntityHandle<ArcOfCircle>,
    pub arc_b: EntityHandle<ArcOfCircle>,
    pub difference: f64,
}

impl ArcArcDifference {
    pub fn new(
        group: Group,
        arc_a: EntityHandle<ArcOfCircle>,
        arc_b: EntityHandle<ArcOfCircle>,
        difference: f64,
    ) -> Self {
        Self {
            group,
            arc_a,
            arc_b,
            difference,
        }
    }
}

impl AsConstraintData for ArcArcDifference {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_ARC_DIFFERENCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc_a.handle(), self.arc_b.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.difference)
    }
}

impl From<Slvs_Constraint> for ArcArcDifference {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            arc_a: EntityHandle::new(value.entityA),
            arc_b: EntityHandle::new(value.entityB),
            difference: value.valA,
        }
    }
}
