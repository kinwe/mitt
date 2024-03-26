use crate::domain::table::Friendships;
rbatis::crud!(Friendships {});
impl_select!(Friendships{select_all_by_userid(user_id:i64) => "`where user_id = #{user_id} and status = 1`"});
impl_select!(Friendships{select_all_by_id(user_id:i64,friend_id:i64)-> Option => "`where user_id = #{user_id} and friend_id = #{friend_id}`"});
impl_update!(Friendships{update_by_user_id(user_id:i64, friend_id: i64) => "`where user_id = #{user_id} and friend_id = #{friend_id}`"});