use crate::Json;

pub trait Block {
    fn ty() -> String
    where
        Self: Sized;

    fn to_json(&self, json: &mut Json);
}

/*
impl<T: Block + Clone> Block for Vec<T> {
    fn ty() -> String {
        T::ty()
    }

    fn to_json(&self) -> Value {
        let mut v = Vec::new();
        for b in self {
            v.push(b.to_json());
        }
        Value::Array(v)
    }
}

impl<'a, T: Block + Clone> Block for &'a [T] {
    fn ty() -> String {
        T::ty()
    }

    fn to_json(&self) -> Value {
        let mut v = Vec::new();
        for b in self {
            v.push(b.to_json());
        }
        Value::Array(v)
    }
}
*/
