use simple_logger::SimpleLogger;
use trylog::macros::*;

fn main() {
    SimpleLogger::new().init().unwrap();

    assert_eq!(inspect_or_log!(None::<()>, "It is"), None);

    assert_eq!(unwrap_or_default_log!(None::<i32>, 114514), 0);

    assert_eq!(unwrap_or_log!(None::<i32>, format!("It is")), 0);
}
