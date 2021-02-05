const ID_SIZE: usize = 16;
pub type Identifier = [char; ID_SIZE];

fn id(s: &str) -> Identifier {
    let mut id = [char::default(); ID_SIZE];

    for (i, c) in s.chars().take(ID_SIZE).enumerate() {
        id[i] = c;
    }

    id
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Id {
    id: Identifier,
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self { id: id(s) }
    }
}
