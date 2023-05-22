use serde::{Deserialize, Serialize};

use super::{AsConstraintData, ConstraintHandle};
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_FACE_DISTANCE},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtFaceDistance {
    pub group: Group,
    pub point: PointHandle,
    pub plane: EntityHandle<Workplane>,
    pub distance: f64,
}

impl PtFaceDistance {
    pub fn new(
        group: Group,
        point: PointHandle,
        plane: EntityHandle<Workplane>,
        distance: f64,
    ) -> Self {
        Self {
            group,
            point,
            plane,
            distance,
        }
    }
}

impl AsConstraintData for PtFaceDistance {
    fn from_system(
        sys: &System,
        constraint_handle: &ConstraintHandle<Self>,
    ) -> Result<Self, &'static str> {
        let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;
        let point = (*sys.slvs_entity(slvs_constraint.ptA)?).try_into()?;

        Ok(Self {
            group: Group(slvs_constraint.group),
            point,
            plane: EntityHandle::new(slvs_constraint.entityA),
            distance: slvs_constraint.valA,
        })
    }

    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_FACE_DISTANCE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.point.handle()])
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.plane.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.distance)
    }
}
