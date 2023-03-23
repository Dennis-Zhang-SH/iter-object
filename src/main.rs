use iter_object::iter_object;
use iter_object::IterObject;

pub mod db {
    pub mod user {
        pub mod str {
            pub fn set(_s: String) -> () {}
        }
        pub mod n {
            pub fn set(_n: i32) -> () {}
        }
    }
}

#[iter_object(db::user)]
#[derive(Default)]
struct Foo {
    str: Option<String>,
    n: Option<i32>,
}

fn main() {
    let f = Foo::default();
    assert!(f.to_params().len() == 0);
    let mut f = Foo::default();
    f.str = Some("str".into());
    assert!(f.to_params().len() == 1);
}
