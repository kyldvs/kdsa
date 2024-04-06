#[derive(Debug)]
pub struct Node<T> {
  data: T,
  children: Vec<Node<T>>,
}
