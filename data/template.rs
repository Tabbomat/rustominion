use std::collections::HashMap;

#[readonly::make]
struct SomeClass {
    pub attr: u64,
}

impl SomeClass {
    pub fn new() -> Self {
        Self { attr: 0 }
    }

    pub fn something(&self) {}
}

// finished: true/false
// old_source: something_compressed
// depending on context, print diffs or source



// enum/static/export
enum SomeEnum {

}

pub type SomeEnumMap = HashMap<SomeEnum, SomeClass>;