use crate::target_union::TargetUnion;

#[derive(Default, Debug, Clone)]
pub struct Cname {
    // struct {
    //     unsigned uid: i32;
    //     is_name_ptr: i32;  /* disciminates target union */
    //   } cname;
    pub target: TargetUnion,
    pub uid: i32,
    pub is_name_ptr: i32,
}
