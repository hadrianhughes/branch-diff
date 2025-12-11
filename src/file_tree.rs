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
        change_kind: FileChangeKind,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum FileChangeKind {
    Creation = 0,
    Deletion = 1,
    Change = 2,
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

    pub fn insert_file(&mut self, path: &str, changes: Vec<Change>, change_kind: FileChangeKind) {
        let mut segments = path.split('/').peekable();
        let mut current_tree = self;

        while let Some(seg) = segments.next() {
            match current_tree {
                Self::Directory { children, .. } => {
                    if segments.peek().is_none() {
                        children.push(Self::File { name: seg.to_string(), changes, change_kind });
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

pub struct FileTreeFilesItem<'a> {
    pub name: &'a str,
    pub changes: &'a Vec<Change>,
    pub change_kind: &'a FileChangeKind,
}

impl<'a> Iterator for FileTreeFilesIter<'a> {
    type Item = FileTreeFilesItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                FileTree::Directory { children, .. } => {
                    for child in children {
                        self.stack.push(child);
                    }
                },
                FileTree::File { name, changes, change_kind } => {
                    return Some(FileTreeFilesItem { name, changes, change_kind });
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

pub struct FileTreeItem<'a> {
    pub node: &'a FileTree,
    pub depth: usize,
}

impl<'a> Iterator for FileTreeIter<'a> {
    type Item = FileTreeItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (node, depth) = self.stack.pop()?;

        if let FileTree::Directory { children, .. } = node {
            for child in children {
                self.stack.push((child, depth + 1));
            }
        }

        Some(FileTreeItem { node, depth })
    }
}
