use systemconfiguration::ConsoleUser;

#[test]
fn console_user_is_queryable() {
    if let Some(user) = ConsoleUser::current() {
        assert_ne!(user.name(), "");
        let _ = user.uid();
        let _ = user.gid();
    }
}
