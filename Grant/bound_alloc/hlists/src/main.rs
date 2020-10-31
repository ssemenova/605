mod hlist;

use hlist::{HCons, HNil, HList};

fn create_hlist() -> HCons<i32, HCons<i32, HCons<i32, HNil>>> {
    HNil.prepend(3).prepend(2).prepend(1)
}

fn main() {
    let hlist = create_hlist();
    println!("Hello HList {:?}!", hlist);
}
