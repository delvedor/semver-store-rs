mod node;

pub struct SemverStore<T> {
    tree: node::Node<T>,
}

impl<T> SemverStore<T> {
    pub fn new() -> Self {
        SemverStore {
            tree: node::Node::new(0),
        }
    }

    pub fn insert(&mut self, version: &String, store: T) {
        let semver: Vec<&str> = version.split('.').collect();
        let mut current_node = &mut self.tree;
        for v in semver {
            let version_number = v.parse::<u32>().unwrap();
            let node = node::Node::new(version_number);
            current_node = current_node.add_child(node);
        }
        current_node.set_store(store);
    }

    pub fn get(&mut self, version: &String) -> Option<&T> {
        let semver: Vec<&str> = version.split('.').collect();
        let major = semver.get(0).unwrap();
        let minor = semver.get(1).unwrap();
        let patch = semver.get(2);

        if let &"x" = minor {
            return self
                .tree
                .get_child(int(&major))
                .and_then(|major| major.get_max_child())
                .and_then(|minor| minor.get_max_child())
                .and_then(|patch| patch.store.as_ref());
        }

        if patch.is_none() {
            return self
                .tree
                .get_child(int(&major))
                .and_then(|major| major.get_child(int(&minor)))
                .and_then(|minor| minor.get_max_child())
                .and_then(|patch| patch.store.as_ref());
        }

        if let &"x" = patch.unwrap() {
            return self
                .tree
                .get_child(int(&major))
                .and_then(|major| major.get_child(int(&minor)))
                .and_then(|minor| minor.get_max_child())
                .and_then(|patch| patch.store.as_ref());
        }

        self.tree
            .get_child(int(&major))
            .and_then(|major| major.get_child(int(&minor)))
            .and_then(|minor| minor.get_child(int(&patch.unwrap())))
            .and_then(|patch| patch.store.as_ref())
    }

    pub fn contains_key(&mut self, version: &String) -> bool {
        match self.get(version) {
            Some(_v) => true,
            None => false,
        }
    }

    pub fn remove(&mut self, version: &String) -> Option<T> {
        if self.contains_key(&version) == false {
            return None;
        }

        let semver: Vec<&str> = version.split('.').collect();
        let major = semver.get(0).unwrap();
        let minor = semver.get(1).unwrap();
        let patch = semver.get(2);

        let major_node = self.tree.get_child(int(&major)).unwrap();

        // eg: '1.x'
        if let &"x" = minor {
            let minor_prefix = major_node
                .get_max_child()
                .and_then(|minor| Some(minor.prefix))
                .unwrap();

            let patch_prefix = major_node
                .get_child(minor_prefix)
                .and_then(|minor| minor.get_max_child())
                .and_then(|patch| Some(patch.prefix))
                .unwrap();

            let patch_node = major_node
                .get_child(minor_prefix)
                .and_then(|minor| minor.remove_child(patch_prefix));

            self.tree.remove_child(int(&major));

            return patch_node.and_then(|node| node.store);
        }

        // eg: '1.2'
        if patch.is_none() {
            let patch_prefix = major_node
                .get_child(int(&minor))
                .and_then(|minor| minor.get_max_child())
                .and_then(|patch| Some(patch.prefix))
                .unwrap();

            let patch_node = major_node
                .get_child(int(&minor))
                .and_then(|minor| minor.remove_child(patch_prefix));

            major_node.remove_child(int(&minor));
            if major_node.children.len() == 0 {
                self.tree.remove_child(int(&major));
            }

            return patch_node.and_then(|node| node.store);
        }

        let patch = patch.unwrap();

        // eg: '1.2.x'
        if let &"x" = patch {
            let patch_prefix = major_node
                .get_child(int(&minor))
                .and_then(|minor| minor.get_max_child())
                .and_then(|patch| Some(patch.prefix))
                .unwrap();

            let patch_node = major_node
                .get_child(int(&minor))
                .and_then(|minor| minor.remove_child(patch_prefix));

            major_node.remove_child(int(&minor));
            if major_node.children.len() == 0 {
                self.tree.remove_child(int(&major));
            }

            return patch_node.and_then(|node| node.store);
        }

        // eg: '1.2.3'
        let patch_node = major_node
            .get_child(int(&minor))
            .and_then(|minor| minor.remove_child(int(&patch)));

        let minor_node = major_node.get_child(int(&minor)).unwrap();

        // if we removed the last child, we should
        // also remove the parent node
        if minor_node.children.len() == 0 {
            major_node.remove_child(int(&minor));
        }
        if major_node.children.len() == 0 {
            self.tree.remove_child(int(&major));
        }

        return patch_node.and_then(|node| node.store);
    }

    pub fn empty(&mut self) {
        self.tree = node::Node::new(0);
    }
}

fn int(str: &str) -> u32 {
    str.parse::<u32>().unwrap()
}

#[cfg(test)]
mod semver_store_tests {
    use super::SemverStore;

    #[test]
    fn create_a_store() {
        let store = SemverStore::<i32>::new();
        assert_eq!(store.tree.prefix, 0);
    }

    #[test]
    fn store_a_string() {
        let mut store = SemverStore::<String>::new();
        store.insert(&"1.0.0".to_string(), "hello".to_string());
        assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &"hello");
    }

    #[test]
    fn not_found() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.0".to_string(), 1);
        assert_eq!(store.get(&"1.2.0".to_string()), None);
        assert_eq!(store.get(&"1.0.1".to_string()), None);
        assert_eq!(store.get(&"1.1.x".to_string()), None);
        assert_eq!(store.get(&"2.0.0".to_string()), None);
        assert_eq!(store.get(&"2.1".to_string()), None);
        assert_eq!(store.get(&"2.x".to_string()), None);
    }

    #[test]
    fn store_multiple_values() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.0".to_string(), 1);
        store.insert(&"1.1.0".to_string(), 2);
        store.insert(&"1.2.0".to_string(), 3);
        store.insert(&"1.3.0".to_string(), 4);

        // the node with prefix `1` should have 4 children
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 4);
        assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &1);
        assert_eq!(store.get(&"1.1.0".to_string()).unwrap(), &2);
        assert_eq!(store.get(&"1.2.0".to_string()).unwrap(), &3);
        assert_eq!(store.get(&"1.3.0".to_string()).unwrap(), &4);
    }

    #[test]
    fn store_has_key() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.0".to_string(), 1);
        store.insert(&"1.1.0".to_string(), 2);
        store.insert(&"1.2.0".to_string(), 3);
        store.insert(&"1.3.0".to_string(), 4);

        assert_eq!(store.contains_key(&"1.1.0".to_string()), true);
        assert_eq!(store.contains_key(&"1.2.3".to_string()), false);
    }

    #[test]
    fn store_multiple_values_and_multiple_prefixes() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.1.0".to_string(), 11);
        store.insert(&"1.2.0".to_string(), 12);
        store.insert(&"1.3.0".to_string(), 13);

        store.insert(&"2.0.0".to_string(), 21);
        store.insert(&"2.1.0".to_string(), 22);
        store.insert(&"2.2.0".to_string(), 23);
        store.insert(&"2.3.0".to_string(), 24);

        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 3);
        assert_eq!(store.tree.children.get(&2).unwrap().children.len(), 4);

        assert_eq!(store.get(&"1.1.0".to_string()).unwrap(), &11);
        assert_eq!(store.get(&"1.2.0".to_string()).unwrap(), &12);
        assert_eq!(store.get(&"1.3.0".to_string()).unwrap(), &13);

        assert_eq!(store.get(&"2.0.0".to_string()).unwrap(), &21);
        assert_eq!(store.get(&"2.1.0".to_string()).unwrap(), &22);
        assert_eq!(store.get(&"2.2.0".to_string()).unwrap(), &23);
        assert_eq!(store.get(&"2.3.0".to_string()).unwrap(), &24);
    }

    #[test]
    fn delete_stored_values() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.0".to_string(), 1);
        store.insert(&"1.1.0".to_string(), 2);
        store.insert(&"1.2.0".to_string(), 3);
        store.insert(&"1.3.0".to_string(), 4);

        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 4);
        assert_eq!(store.remove(&"1.2.0".to_string()), Some(3));
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 3);
        assert_eq!(store.remove(&"2.2.0".to_string()), None);
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 3);
        assert_eq!(store.remove(&"1.4.2".to_string()), None);
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 3);
    }

    #[test]
    fn delete_minor_wildcard_shortcut() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.0".to_string(), 1);
        store.insert(&"1.1.0".to_string(), 2);
        store.insert(&"1.1.1".to_string(), 3);
        store.insert(&"1.1.2".to_string(), 4);
        store.insert(&"1.2.0".to_string(), 5);
        store.insert(&"1.2.1".to_string(), 6);
        store.insert(&"1.2.2".to_string(), 7);

        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 3);
        assert_eq!(store.remove(&"1.1.x".to_string()), Some(4));
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 2);
        assert_eq!(store.remove(&"1.2".to_string()), Some(7));
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 1);
        assert_eq!(store.remove(&"1.3".to_string()), None);
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 1);
        assert_eq!(store.remove(&"1.3.x".to_string()), None);
        assert_eq!(store.tree.children.get(&1).unwrap().children.len(), 1);
    }

    #[test]
    fn delete_major_wildcard_shortcut() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.0".to_string(), 1);
        store.insert(&"1.1.0".to_string(), 2);
        store.insert(&"2.0.0".to_string(), 3);
        store.insert(&"2.1.0".to_string(), 4);
        store.insert(&"3.0.0".to_string(), 5);
        store.insert(&"3.1.0".to_string(), 6);

        assert_eq!(store.tree.children.len(), 3);
        assert_eq!(store.remove(&"1.x".to_string()), Some(2));
        assert_eq!(store.tree.children.len(), 2);
        assert_eq!(store.remove(&"2.x".to_string()), Some(4));
        assert_eq!(store.tree.children.len(), 1);
        assert_eq!(store.remove(&"4.x".to_string()), None);
        assert_eq!(store.tree.children.len(), 1);
    }

    #[test]
    fn get_patch_wildcard_shortcut() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.1".to_string(), 1);
        store.insert(&"1.0.2".to_string(), 2);
        store.insert(&"1.0.3".to_string(), 3);
        store.insert(&"2.0.0".to_string(), 4);

        assert_eq!(store.get(&"1.0.x".to_string()).unwrap(), &3);
        assert_eq!(store.get(&"1.0".to_string()).unwrap(), &3);
    }

    #[test]
    fn get_minor_wildcard() {
        let mut store = SemverStore::<i32>::new();
        store.insert(&"1.0.1".to_string(), 1);
        store.insert(&"1.1.2".to_string(), 2);
        store.insert(&"1.2.3".to_string(), 3);
        store.insert(&"2.0.0".to_string(), 4);

        assert_eq!(store.get(&"1.1".to_string()).unwrap(), &2);
        assert_eq!(store.get(&"1.x".to_string()).unwrap(), &3);
    }

    #[test]
    fn empty_store() {
        let mut store = SemverStore::<String>::new();
        store.insert(&"1.0.0".to_string(), "hello".to_string());
        assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &"hello");
        store.empty();
        assert_eq!(store.get(&"1.0.0".to_string()), None);
    }
}
