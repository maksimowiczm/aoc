#[derive(Debug)]
pub struct Node<'a> {
    pub(crate) right: &'a str,
    pub(crate) left: &'a str,
}

impl Node<'_> {
    pub fn next(&self, direction: &char) -> &str {
        match direction {
            'R' => self.right,
            'L' => self.left,
            _ => panic!(),
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Node<'a> {
    fn from((right, left): (&'a str, &'a str)) -> Self {
        Node { right, left }
    }
}
