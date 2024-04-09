use std::collections::HashMap;

#[derive(Debug)]
pub enum BinaryTreeError {
  /// This error occurs when setting a value at a position with no valid
  /// parent node.
  MissingParent,
}

/// A BinaryTree structure backed by a [`HashMap`].
///
/// [`HashMap`]: std::collections::HashMap
#[derive(Debug)]
pub struct BinaryTree<T> {
  map: HashMap<usize, T>,
}

// Notice that a related node can be found in constant time:
// n.left = n * 2
// n.right = n * 2 + 1
// n.parent = n / 2 (integer divison)
//
//            1
//      2           3
//   4     5     6     7
// 08 09 10 11 12 13 14 15

#[derive(Debug)]
pub struct ViewMut<'a, T> {
  tree: &'a mut BinaryTree<T>,
  i: usize,
}

#[derive(Debug)]
pub struct View<'a, T> {
  tree: &'a BinaryTree<T>,
  i: usize,
}

impl<T> Default for BinaryTree<T> {
  fn default() -> Self {
    Self {
      map: HashMap::new(),
    }
  }
}

impl<T> BinaryTree<T> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      map: HashMap::with_capacity(capacity),
    }
  }

  pub fn view(&self) -> View<T> {
    View { tree: self, i: 1 }
  }

  pub fn view_mut(&mut self) -> ViewMut<T> {
    ViewMut { tree: self, i: 1 }
  }

  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  pub fn len(&self) -> usize {
    self.map.len()
  }
}

impl<'a, T> ViewMut<'a, T> {
  pub fn get(&self) -> Option<&T> {
    self.tree.map.get(&self.i)
  }

  pub fn set(&mut self, data: T) -> Result<Option<T>, BinaryTreeError> {
    // The parent can always be found through integer division. The root is
    // at index 1. If this reaches 0 it's an invalid index.
    let parent = self.i / 2;
    // Nodes can be set if they are the root, or they have a valid parent.
    if self.i == 1 || self.tree.map.contains_key(&parent) {
      let prev_data = self.tree.map.insert(self.i, data);
      Ok(prev_data)
    } else {
      Err(BinaryTreeError::MissingParent)
    }
  }

  pub fn left(&'a mut self) -> Self {
    Self {
      tree: self.tree,
      i: self.i * 2,
    }
  }

  pub fn right(&'a mut self) -> Self {
    Self {
      tree: self.tree,
      i: self.i * 2 + 1,
    }
  }

  pub fn parent(&'a mut self) -> Self {
    Self {
      tree: self.tree,
      // Cannot go up if already at the root. Just return the root again.
      i: if self.i == 1 { self.i } else { self.i / 2 },
    }
  }
}

impl<'a, T> View<'a, T> {
  pub fn get(&self) -> Option<&T> {
    self.tree.map.get(&self.i)
  }

  pub fn left(&'a self) -> Self {
    Self {
      tree: self.tree,
      i: self.i * 2,
    }
  }

  pub fn right(&'a self) -> Self {
    Self {
      tree: self.tree,
      i: self.i * 2 + 1,
    }
  }

  pub fn parent(&'a self) -> Self {
    Self {
      tree: self.tree,
      // Cannot go up if already at the root. Just return the root again.
      i: if self.i == 1 { self.i } else { self.i / 2 },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::BinaryTree;

  #[test]
  fn it_works() {
    let mut t = BinaryTree::<&str>::new();
    assert_eq!(0, t.len());

    // Add some items.
    let mut view = t.view_mut();
    let _ = view.set("hello");
    let mut view = view.right();
    let _ = view.set("world");

    let mut v = t.view_mut();
    assert_eq!(Some("hello"), v.get().copied());
    let v = v.right();
    assert_eq!(Some("world"), v.get().copied());
    assert_eq!(2, t.len());
  }

  #[test]
  fn it_works_view() {
    let mut t = BinaryTree::<&str>::new();
    assert_eq!(0, t.len());

    // Add some items.
    let mut view = t.view_mut();
    let _ = view.set("hello");
    let mut view = view.left();
    let _ = view.set("world");
    let mut view = view.right();
    let _ = view.set("i'm");
    let mut view = view.left();
    let _ = view.set("a");
    let mut view = view.parent();
    let mut view = view.right();
    let _ = view.set("tree");

    assert_eq!(5, t.len());

    let v = t.view();
    assert_eq!(Some("hello"), v.get().copied());
    assert_eq!(Some("world"), v.left().get().copied());
    assert_eq!(Some("i'm"), v.left().right().get().copied());
    assert_eq!(Some("a"), v.left().right().left().get().copied());
    assert_eq!(Some("tree"), v.left().right().right().get().copied());
  }
}
