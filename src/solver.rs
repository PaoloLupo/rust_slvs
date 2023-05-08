use serde::{Deserialize, Serialize};

use crate::{
    bindings::{
        Slvs_hConstraint, SLVS_RESULT_DIDNT_CONVERGE, SLVS_RESULT_INCONSISTENT,
        SLVS_RESULT_TOO_MANY_UNKNOWNS,
    },
    constraint::AsConstraintHandle,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SolveOkay {
    pub dof: i32,
}

#[derive(Debug, Serialize)]
pub struct SolveFail {
    pub dof: i32,
    pub reason: FailReason,
    pub failed_constraints: Vec<Slvs_hConstraint>,
}

impl SolveFail {
    pub fn constraint_did_fail<T: AsConstraintHandle>(&self, constraint: &T) -> bool {
        self.failed_constraints
            .iter()
            .any(|&constraint_h| constraint_h == constraint.handle())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FailReason {
    Inconsistent,
    DidntConverge,
    TooManyUnknowns,
}

impl TryFrom<i32> for FailReason {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, &'static str> {
        match value as _ {
            SLVS_RESULT_INCONSISTENT => Ok(Self::Inconsistent),
            SLVS_RESULT_DIDNT_CONVERGE => Ok(Self::DidntConverge),
            SLVS_RESULT_TOO_MANY_UNKNOWNS => Ok(Self::TooManyUnknowns),
            _ => Err("Result must be of values 1, 2, or 3."),
        }
    }
}
