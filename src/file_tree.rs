use crate::state::Change;

#[derive(Debug)]
pub enum FileTree {
    Directory {
        name: String,
        children: Vec<FileTree>,
    },
    File {
        name: String,
        changes: Vec<Change>,
    },
}

impl FileTree {
    pub fn new(root_dir: &str) -> Self {
        Self::Directory {
            name: root_dir.to_string(),
            children: Vec::new(),
        }
    }

    pub fn iter(&self) -> FileTreeIter<'_> {
        FileTreeIter::new(self)
    }

    pub fn iter_files(&self) -> FileTreeFilesIter<'_> {
        FileTreeFilesIter::new(self)
    }

    pub fn insert_file(&mut self, path: &str, changes: Vec<Change>) {
        let mut segments = path.split('/').peekable();
        let mut current_tree = self;

        while let Some(seg) = segments.next() {
            match current_tree {
                Self::Directory { children, .. } => {
                    if segments.peek().is_none() {
                        children.push(Self::File { name: seg.to_string(), changes });
                        return;
                    }

                    let sub_dir = children.iter().position(|child| match child {
                        FileTree::Directory { name, .. } => name == seg,
                        _ => false,
                    });

                    match sub_dir {
                        Some(idx) => {
                            current_tree = &mut children[idx];
                        },
                        None => {
                            children.push(FileTree::new(seg));
                            current_tree = children.last_mut().unwrap();
                        },
                    }
                },
                Self::File { .. } => {
                    panic!("encountered a `File` node for `current_tree` before last iteration");
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct FileTreeFilesIter<'a> {
    stack: Vec<&'a FileTree>,
}

impl<'a> FileTreeFilesIter<'a> {
    pub fn new(root: &'a FileTree) -> Self {
        FileTreeFilesIter { stack: vec![root] }
    }
}

impl<'a> Iterator for FileTreeFilesIter<'a> {
    type Item = (&'a str, &'a Vec<Change>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                FileTree::Directory { children, .. } => {
                    for child in children {
                        self.stack.push(child);
                    }
                },
                FileTree::File { name, changes } => {
                    return Some((name, changes));
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct FileTreeIter<'a> {
    stack: Vec<(&'a FileTree, usize)>, // usize = depth
}

impl<'a> FileTreeIter<'a> {
    pub fn new(root: &'a FileTree) -> Self {
        FileTreeIter { stack: vec![(root, 0)] }
    }
}

impl<'a> Iterator for FileTreeIter<'a> {
    type Item = (&'a FileTree, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (node, depth) = self.stack.pop()?;

        if let FileTree::Directory { children, .. } = node {
            for child in children {
                self.stack.push((child, depth + 1));
            }
        }

        Some((node, depth))
    }
}
