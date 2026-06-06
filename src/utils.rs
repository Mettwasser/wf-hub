use notify_rust::Notification;

pub fn notification() -> Notification {
    let mut n = Notification::new();
    n.appname("Warframe Hub");
    n
}
