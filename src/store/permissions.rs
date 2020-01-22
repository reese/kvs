pub enum Permissions {
    Allowed,
    NotAllowed,
}

impl Permissions {
    pub fn is_allowed(&self) -> bool {
        match self {
            Permissions::Allowed => true,
            Permissions::NotAllowed => false,
        }
    }
}
