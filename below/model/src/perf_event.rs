// Copyright (c) Facebook, Inc. and its affiliates.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize, below_derive::Queriable)]
pub struct PerfEventModel {
    //#[queriable(subquery)]
    pub events: BTreeMap<String, u64>,
    //pub events: BTreeMap<String, SinglePerfEventModel>,
}

impl PerfEventModel {
    pub fn new() -> Self {
        PerfEventModel {
            events: BTreeMap::from([
                ("foo".to_string(), 123),
                ("bar".to_string(), 456),
                ("baz".to_string(), 789),
                //("foo".to_string(), SinglePerfEventModel::new("foo", 123)),
                //("bar".to_string(), SinglePerfEventModel::new("bar", 456)),
                //("baz".to_string(), SinglePerfEventModel::new("baz", 789)),
            ]),
        }
    }
    pub fn new_from(events: BTreeMap<String, u64>) -> Self {
        PerfEventModel { events }
    }
}

impl From<Field> for PerfEventModel {
    fn from(field: Field) -> PerfEventModel {
        match field {
            Field::BTreeMap(v) => PerfEventModel::new_from(v),
            //Field::Str(v) => BTreeMap::from([(v, 0)]),
            _ => panic!("Operation for unsupported types"),
        }
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    below_derive::Queriable
)]
pub struct SinglePerfEventModel {
    pub event: String,
    pub value: Option<u64>,
}

impl SinglePerfEventModel {
    fn new(event: &str, sample: u64) -> SinglePerfEventModel {
        SinglePerfEventModel {
            event: event.to_string(),
            value: Some(sample),
        }
    }
}
