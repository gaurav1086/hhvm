// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
use hhas_type::Info;
use hhbc_id_rust::record;
use runtime::TypedValue;

pub struct Field(pub String, pub Info, pub Option<TypedValue>);

pub struct HhasRecord<'a> {
    pub name: record::Type<'a>,
    pub is_abstract: bool,
    pub base: Option<record::Type<'a>>,
    pub fields: Vec<Field>,
}
