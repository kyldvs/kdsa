use std::cmp::Ordering;

#[derive(Debug)]
pub struct BST<T>
where
    T: PartialOrd,
{
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> BST<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn root(&self) -> Option<&T> {
        Some(&self.root.as_ref()?.data)
    }

    pub fn insert(&mut self, item: T) {
        if let Some(n) = &mut self.root {
            let _ = n.insert(item);
        } else {
            self.root = Some(Box::new(Node {
                data: item,
                left: None,
                right: None,
            }))
        }
    }

    pub fn inorder(&self) -> Vec<&T> {
        let mut out: Vec<&T> = Vec::new();
        if let Some(n) = &self.root {
            n.inorder(&mut out);
        }
        out
    }
}

impl<T> Node<T>
where
    T: PartialOrd,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, item: T) -> Option<()> {
        match PartialOrd::partial_cmp(&item, &self.data)? {
            Ordering::Less | Ordering::Equal => {
                if let Some(n) = &mut self.left {
                    n.insert(item)
                } else {
                    self.left = Some(Box::new(Node::new(item)));
                    Some(())
                }
            }
            Ordering::Greater => {
                if let Some(n) = &mut self.right {
                    n.insert(item)
                } else {
                    self.right = Some(Box::new(Node::new(item)));
                    Some(())
                }
            }
        }
    }

    fn inorder<'a>(&'a self, res: &mut Vec<&'a T>) {
        if let Some(n) = &self.left {
            n.inorder(res);
        }
        res.push(&self.data);
        if let Some(n) = &self.right {
            n.inorder(res);
        }
    }
}

impl<T> Default for BST<T>
where
    T: PartialOrd,
{
    fn default() -> Self {
        Self { root: None }
    }
}

#[cfg(test)]
mod tests {
    use super::BST;

    #[test]
    fn it_works() {
        let mut tree = BST::new();
        tree.insert(5);
        tree.insert(1);
        tree.insert(6);
        tree.insert(3);
        let v = tree.inorder().iter().map(|i| **i).collect::<Vec<_>>();
        assert_eq!(vec![1, 3, 5, 6], v)
    }
}
