use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::{fmt::Debug, marker::PhantomData};

use crate::{
    bindings::{Slvs_Entity, Slvs_hEntity, Slvs_hGroup},
    element::{AsHandle, TypeInfo},
    target::AsTarget,
};

mod point;
pub use point::Point;
mod normal;
pub use normal::Normal;
mod distance;
pub use distance::Distance;
mod workplane;
pub use workplane::Workplane;
mod line_segment;
pub use line_segment::LineSegment;
mod cubic;
pub use cubic::Cubic;
mod circle;
pub use circle::Circle;
mod arc_of_circle;
pub use arc_of_circle::ArcOfCircle;

pub trait AsEntityHandle: AsHandle + Send + Sync {
    fn phantom_type(&self) -> String;
}

impl Serialize for Box<dyn AsEntityHandle> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Entity", 2)?;
        state.serialize_field("handle", &self.handle())?;
        state.serialize_field("type", &self.phantom_type())?;
        state.end()
    }
}

pub trait AsEntityData: Copy + TypeInfo + Send + Sync {
    fn type_(&self) -> i32;
    fn workplane(&self) -> Option<Slvs_hEntity>;
    fn group(&self) -> Slvs_hGroup;

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        None
    }
    fn normal(&self) -> Option<Slvs_hEntity> {
        None
    }
    fn distance(&self) -> Option<Slvs_hEntity> {
        None
    }
    fn param_vals(&self) -> Option<Vec<f64>> {
        None
    }
}

pub trait FromSlvsEntity<T: AsTarget>: AsEntityData {
    fn from(slvs_entity: Slvs_Entity) -> Self;

    fn set_vals(&mut self, _vals: Vec<f64>) {}
}

pub trait As2dProjectionTarget: AsEntityData {}
pub trait AsArc: AsEntityData {}
pub trait AsCubic: AsEntityData {}
pub trait AsCurve: AsEntityData {}
pub trait AsLineSegment: AsEntityData {}
pub trait AsPoint: AsEntityData {}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct EntityHandle<T: AsEntityData> {
    pub handle: u32,
    pub(super) phantom: PhantomData<T>,
}

impl<T: AsEntityData> EntityHandle<T> {
    pub(super) fn new(handle: u32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<T: AsEntityData> Debug for EntityHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity: {{h: {}, type: {}}}", self.handle, T::type_of())
    }
}

impl<T: AsEntityData> AsHandle for EntityHandle<T> {
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl<T: AsEntityData> AsEntityHandle for EntityHandle<T> {
    fn phantom_type(&self) -> String {
        T::type_of()
    }
}
