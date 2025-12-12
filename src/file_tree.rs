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
        scroll_start: usize,
        hunks: Vec<usize>,
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

    pub fn name(&self) -> &String {
        match self {
            Self::Directory { name, .. } => name,
            Self::File { name, .. } => name,
        }
    }

    pub fn iter(&self) -> FileTreeIter<'_> {
        FileTreeIter::new(self)
    }

    pub fn iter_files(&self) -> FileTreeFilesIter<'_> {
        FileTreeFilesIter::new(self)
    }

    pub fn insert_file(
        &mut self,
        path: &str,
        changes: Vec<Change>,
        change_kind: FileChangeKind,
        hunks: Vec<usize>,
        scroll_start: usize,
    ) {
        let mut segments = path.split('/').peekable();
        let mut current_tree = self;

        while let Some(seg) = segments.next() {
            match current_tree {
                Self::Directory { children, .. } => {
                    if segments.peek().is_none() {
                        children.push(Self::File {
                            name: seg.to_string(),
                            changes,
                            change_kind,
                            scroll_start,
                            hunks,
                        });
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

    pub fn sort(&mut self) {
        if let Self::Directory { children, .. } = self {
            children.sort_by(|a,b| a.name().cmp(&b.name()));
            children.reverse();

            for child in children {
                child.sort();
            }
        }
    }

    pub fn get_next_hunk(&self, start_at: usize) -> Option<usize> {
        let mut prev_hunk_idx = 0;

        for node in self.iter_files() {
            for hunk_idx in node.hunks {
                if prev_hunk_idx <= start_at && *hunk_idx > start_at {
                    return Some(*hunk_idx);
                }

                prev_hunk_idx = *hunk_idx;
            }
        }

        None
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

#[derive(Debug)]
pub struct FileTreeFilesItem<'a> {
    pub name: &'a str,
    pub changes: &'a Vec<Change>,
    pub change_kind: &'a FileChangeKind,
    pub hunks: &'a Vec<usize>,
    pub scroll_start: usize,
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
                FileTree::File {
                    name,
                    changes,
                    change_kind,
                    hunks,
                    scroll_start,
                } => {
                    return Some(FileTreeFilesItem {
                        name,
                        changes,
                        change_kind,
                        hunks,
                        scroll_start: *scroll_start,
                    });
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

#[derive(Debug)]
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
