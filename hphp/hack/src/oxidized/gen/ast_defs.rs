// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
//
// @generated SignedSource<<b2ba8876ded22512c6427949c6310ee4>>
//
// To regenerate this file, run:
//   hphp/hack/src/oxidized/regen.sh

use ocamlrep_derive::OcamlRep;
use serde::Deserialize;
use serde::Serialize;

use crate::pos;

pub use crate::shape_map;

pub use pos::Pos;

#[derive(Clone, Debug, Deserialize, OcamlRep, Serialize)]
pub struct Id(pub Pos, pub String);

pub type Pstring = (Pos, String);

#[derive(Clone, Debug, Deserialize, OcamlRep, Serialize)]
pub enum ShapeFieldName {
    SFlitInt(Pstring),
    SFlitStr(Pstring),
    SFclassConst(Id, Pstring),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum Variance {
    Covariant,
    Contravariant,
    Invariant,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum ConstraintKind {
    ConstraintAs,
    ConstraintEq,
    ConstraintSuper,
}

pub type Reified = bool;

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum ClassKind {
    Cabstract,
    Cnormal,
    Cinterface,
    Ctrait,
    Cenum,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum ParamKind {
    Pinout,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum OgNullFlavor {
    OGNullthrows,
    OGNullsafe,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum FunKind {
    FSync,
    FAsync,
    FGenerator,
    FAsyncGenerator,
    FCoroutine,
}

#[derive(Clone, Debug, Deserialize, OcamlRep, Serialize)]
pub enum Bop {
    Plus,
    Minus,
    Star,
    Slash,
    Eqeq,
    Eqeqeq,
    Starstar,
    Diff,
    Diff2,
    Ampamp,
    Barbar,
    LogXor,
    Lt,
    Lte,
    Gt,
    Gte,
    Dot,
    Amp,
    Bar,
    Ltlt,
    Gtgt,
    Percent,
    Xor,
    Cmp,
    QuestionQuestion,
    Eq(Option<Box<Bop>>),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum Uop {
    Utild,
    Unot,
    Uplus,
    Uminus,
    Uincr,
    Udecr,
    Upincr,
    Updecr,
    Usilence,
}
