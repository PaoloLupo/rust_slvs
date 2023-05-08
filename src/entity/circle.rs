use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::{AsArc, AsEntityData, Distance, EntityHandle, FromSlvsEntity, Normal, Point, Workplane};
use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup, SLVS_E_CIRCLE},
    element::{AsHandle, TypeInfo},
    group::Group,
    target::{AsTarget, In3d, OnWorkplane},
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Circle<T: AsTarget> {
    pub group: Group,
    pub workplane: Option<EntityHandle<Workplane>>,
    pub center: EntityHandle<Point<T>>,
    pub radius: EntityHandle<Distance<T>>,
    pub normal: EntityHandle<Normal>,
}

impl Circle<OnWorkplane> {
    pub fn new(
        group: Group,
        workplane: EntityHandle<Workplane>,
        center: EntityHandle<Point<OnWorkplane>>,
        radius: EntityHandle<Distance<OnWorkplane>>,
        normal: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            workplane: Some(workplane),
            center,
            radius,
            normal,
        }
    }
}

impl Circle<In3d> {
    pub fn new(
        group: Group,
        center: EntityHandle<Point<In3d>>,
        radius: EntityHandle<Distance<In3d>>,
        normal: EntityHandle<Normal>,
    ) -> Self {
        Self {
            group,
            workplane: None,
            center,
            radius,
            normal,
        }
    }
}

impl<T: AsTarget> AsArc for Circle<T> {}

impl<T: AsTarget> AsEntityData for Circle<T> {
    fn type_(&self) -> i32 {
        SLVS_E_CIRCLE as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        self.workplane.map(|workplane| workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.center.handle()])
    }

    fn normal(&self) -> Option<Slvs_hEntity> {
        Some(self.normal.handle())
    }

    fn distance(&self) -> Option<Slvs_hEntity> {
        Some(self.radius.handle())
    }
}

impl<T: AsTarget> TypeInfo for Circle<T> {
    fn type_of() -> String {
        format!("Circle<{}>", T::type_of())
    }
}

impl FromSlvsEntity<OnWorkplane> for Circle<OnWorkplane> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            group: Group(slvs_entity.group),
            workplane: Some(EntityHandle::new(slvs_entity.wrkpl)),
            center: EntityHandle::new(slvs_entity.point[0]),
            radius: EntityHandle::new(slvs_entity.distance),
            normal: EntityHandle::new(slvs_entity.normal),
        }
    }
}

impl FromSlvsEntity<In3d> for Circle<In3d> {
    fn from(slvs_entity: Slvs_Entity) -> Self {
        Self {
            group: Group(slvs_entity.group),
            workplane: None,
            center: EntityHandle::new(slvs_entity.point[0]),
            radius: EntityHandle::new(slvs_entity.distance),
            normal: EntityHandle::new(slvs_entity.normal),
        }
    }
}
