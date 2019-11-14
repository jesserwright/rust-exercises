use std::collections::HashMap;

pub struct Cacher<T, I, R>
where
    T: Fn(I) -> R,
{
    calculation: T,
    arg_map: HashMap<I, R>,
}

impl<T, I, R> Cacher<T, I, R>
where
    T: Fn(I) -> R,
{
    pub fn new(calculation: T) -> Cacher<T, I, R> {
        let hmap: HashMap<I, R> = HashMap::new();
        Cacher {
            calculation,
            arg_map: hmap,
        }
    }

    pub fn value(&mut self, arg: I) -> R {
        match self.arg_map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.arg_map.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

// TODO: cacher needs to be able to take any types!
#[test]
fn check_generic_values() {
    let mut c = Cacher::new(|a| a);
}
