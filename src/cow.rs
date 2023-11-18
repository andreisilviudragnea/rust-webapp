use std::borrow::Cow;

trait Storage<'a> {
    fn get_code(&'a self) -> Cow<'a, str>;
}

struct OwnedStorage(String);

impl<'a> Storage<'a> for OwnedStorage {
    fn get_code(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.0.clone())
    }
}

struct BorrowedStorage<'a>(&'a str);

impl<'a> Storage<'a> for BorrowedStorage<'a> {
    fn get_code(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(self.0)
    }
}

fn use_str(str: &str) {
    let borrowed_storage = BorrowedStorage(str);
    assert_eq!(borrowed_storage.get_code(), Cow::<str>::Borrowed(str));
}

#[test]
fn test() {
    let owned_storage = OwnedStorage("123".to_owned());
    assert_eq!(
        owned_storage.get_code(),
        Cow::<str>::Owned("123".to_owned())
    );

    use_str("123");
}
