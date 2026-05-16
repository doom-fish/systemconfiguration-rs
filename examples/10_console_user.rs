use systemconfiguration::ConsoleUser;

fn main() {
    match ConsoleUser::current() {
        Some(user) => println!("console_user={} uid={} gid={}", user.name(), user.uid(), user.gid()),
        None => println!("no console user"),
    }
}
