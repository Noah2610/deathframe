use super::Merge;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Value<T>(Vec<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug;

impl<T> Merge for Value<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    fn merge(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
        self.0.sort_unstable();
    }
}

struct MockValues {
    values: (Value<u8>, Value<u8>),
    merged: Value<u8>,
}

fn get_values() -> MockValues {
    MockValues {
        values: (Value(vec![0, 1]), Value(vec![2, 3])),
        merged: Value(vec![0, 1, 2, 3]),
    }
}

#[test]
fn merge_into_value() {
    let MockValues {
        values: (mut value, other),
        merged,
    } = get_values();

    value.merge(other);
    assert_eq!(value, merged, "Should merge into `Value`s struct");
}

#[test]
fn create_merged_value() {
    let MockValues {
        values: (value, other),
        merged: merged_expect,
    } = get_values();

    let merged = value.merged(other);
    assert_eq!(merged, merged_expect, "Should create merged `Value` struct");
}

#[test]
fn merge_none_into_some() {
    let MockValues { values, merged: _ } = get_values();

    let mut value = Some(values.0.clone());
    let other: Option<Value<u8>> = None;

    value.merge(other);
    assert_eq!(
        value,
        Some(values.0),
        "Nothing should change when merging `None` into `Some`"
    );
}

#[test]
fn merge_some_into_none() {
    let MockValues { values, merged: _ } = get_values();

    let mut value = None;
    let other = Some(values.1.clone());

    value.merge(other);
    assert_eq!(
        value,
        Some(values.1),
        "Value should become `Some` when merging `Some` into `None`"
    );
}

#[test]
fn merge_some_into_some() {
    let MockValues {
        values,
        merged: merged_expect,
    } = get_values();

    {
        let mut value = Some(values.0.clone());
        let other = Some(values.1.clone());

        value.merge(other);
        assert_eq!(
            value,
            Some(merged_expect.clone()),
            "Should merge `Some` into `Some`"
        );
    }

    {
        let value = Some(values.0.clone());
        let other = Some(values.1.clone());

        let merged = value.merged(other);
        assert_eq!(
            merged,
            Some(merged_expect),
            "Should create merged `Some` from merge of `Some` and `Some`"
        );
    }
}
