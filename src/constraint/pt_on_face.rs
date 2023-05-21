use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_PT_ON_FACE},
    element::AsHandle,
    entity::{EntityHandle, PointHandle, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PtOnFace {
    pub group: Group,
    pub point: PointHandle,
    pub plane: EntityHandle<Workplane>,
}

impl PtOnFace {
    pub fn new(group: Group, point: PointHandle, plane: EntityHandle<Workplane>) -> Self {
        Self {
            group,
            point,
            plane,
        }
    }
}

impl AsConstraintData for PtOnFace {
    fn slvs_type(&self) -> i32 {
        SLVS_C_PT_ON_FACE as _
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
}

// impl<P: AsPoint> From<Slvs_Constraint> for PtOnFace<P> {
//     fn from(value: Slvs_Constraint) -> Self {
//         Self {
//             group: Group(value.group),
//             point: EntityHandle::new(value.ptA),
//             plane: EntityHandle::new(value.entityA),
//         }
//     }
// }
