use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Node {
    freq: u128,
    token: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn leaf(token: String, freq: u128) -> Self {
        Self {
            freq,
            token,
            left: None,
            right: None,
        }
    }

    pub fn merge(left: Node, right: Node) -> Self {
        Self {
            freq: left.freq + right.freq,
            token: left.token.clone()+&right.token,
            left: Some(Box::from(left)),
            right: Some(Box::from(right)),
        }
    }

    pub fn left(&self) -> Option<&Node> {
        self.left.as_deref()
    }

    pub fn right(&self) -> Option<&Node> {
        self.right.as_deref()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    // pub fn freq(&self) -> u128 {
    //     self.freq
    // }
}


impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}