use crate::domain::table::Users;
rbatis::crud!(Users {});
impl_select!(Users{select_all_by_name(name:&str) -> Option => "`where username = #{name} limit 1`"});
impl_select!(Users{select_all_by_id(id:i64) -> Option => "`where id = #{id}`"});