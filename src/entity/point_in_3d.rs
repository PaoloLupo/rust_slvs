use crate::binding;

use super::{AsEntity, EntityData};

#[derive(Clone, Copy)]
pub struct PointIn3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl AsEntity for PointIn3d {
    fn type_(&self) -> binding::Slvs_hEntity {
        binding::SLVS_E_POINT_IN_3D
    }

    fn workplane(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [None; 4]
    }

    fn normal(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<binding::Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> [Option<f64>; 4] {
        [Some(self.x), Some(self.y), Some(self.z), None]
    }
}

impl TryFrom<EntityData> for PointIn3d {
    type Error = &'static str;

    fn try_from(value: EntityData) -> Result<Self, Self::Error> {
        if let EntityData::PointIn3d(data) = value {
            Ok(data)
        } else {
            Err("Expected EntityData::PointIn3d")
        }
    }
}
