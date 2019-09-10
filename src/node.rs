use std::collections::HashMap;

pub struct Node<T> {
    pub prefix: u32,
    pub children: HashMap<u32, Node<T>>,
    pub store: Option<T>,
}

impl<T> Node<T> {
    pub fn new(prefix: u32) -> Self {
        Node {
            prefix,
            children: HashMap::new(),
            store: None,
        }
    }

    pub fn get_child(&mut self, prefix: u32) -> Option<&mut Node<T>> {
        self.children.get_mut(&prefix)
    }

    pub fn add_child(&mut self, node: Node<T>) -> &mut Node<T> {
        let prefix = node.prefix;
        if self.children.contains_key(&prefix) == false {
            self.children.insert(prefix, node);
        }
        self.get_child(prefix).unwrap()
    }

    pub fn get_max_child(&mut self) -> Option<&mut Node<T>> {
        match self.children.keys().max() {
            Some(max) => self.get_child(*max),
            None => None,
        }
    }

    pub fn remove_child(&mut self, prefix: u32) -> Option<Node<T>> {
        self.children.remove(&prefix)
    }

    pub fn set_store(&mut self, store: T) {
        self.store = Some(store);
    }
}

#[cfg(test)]
mod semver_store_tests {
    use super::Node;

    #[test]
    fn create_a_node() {
        let mut root = Node::new(1);
        assert_eq!(root.prefix, 1);
        assert_eq!(root.store, None);
        root.set_store(42);
        assert_eq!(root.store, Some(42));
    }

    #[test]
    fn add_a_child() {
        let mut root = Node::new(1);
        let mut node = Node::new(2);
        node.set_store(10);
        // should return a mutable reference of the child
        let child = root.add_child(node);
        assert_eq!(child.store, Some(10));
    }

    #[test]
    fn add_a_child_multiple_times() {
        let mut root = Node::new(1);
        let mut node1 = Node::new(2);
        let mut node2 = Node::new(2);
        node1.set_store(10);
        node2.set_store(11);
        root.add_child(node1);
        root.add_child(node2);

        assert_eq!(root.children.len(), 1);
        match root.get_child(2) {
            Some(child) => assert_eq!(child.store, Some(10)),
            None => panic!("Should have a value"),
        }
    }

    #[test]
    fn remove_a_child() {
        let mut root = Node::new(1);
        let mut node = Node::new(2);
        node.set_store(10);
        root.add_child(node);

        match root.get_child(2) {
            Some(_c) => assert_eq!(_c.store, Some(10)),
            None => panic!("Should have a value"),
        }
        assert_eq!(root.children.len(), 1);

        root.remove_child(2);

        match root.get_child(2) {
            Some(_c) => panic!("Should be empty"),
            None => assert!(true),
        }
        assert_eq!(root.children.len(), 0);
    }

    #[test]
    fn add_child_to_child() {
        let mut root = Node::new(0);
        let mut node1 = Node::new(1);
        node1.set_store(10);
        root.add_child(node1);

        let node2 = Node::new(2);
        let child = root.get_child(1).unwrap();

        child.add_child(node2);

        assert_eq!(child.children.len(), 1);
    }

    #[test]
    fn get_the_max_child() {
        let mut root = Node::new(1);
        let mut node1 = Node::new(2);
        let mut node2 = Node::new(3);
        let mut node3 = Node::new(4);

        node1.set_store(11);
        node2.set_store(12);
        node3.set_store(13);

        root.add_child(node1);
        root.add_child(node2);
        root.add_child(node3);

        assert_eq!(root.children.len(), 3);
        assert_eq!(root.get_max_child().unwrap().prefix, 4);
    }
}
