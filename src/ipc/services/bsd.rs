pub struct BsdUser;

impl ServiceRoot for BsdUser {
    fn name() -> &'static [u8] { b"bsd:u" }

    fn is_domain() -> bool { false }
}
