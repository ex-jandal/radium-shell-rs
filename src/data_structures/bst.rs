use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum CommandAction {
    SystemClear,
    SystemExit,
    DataDisplay,
    SystemCd,
    ExecuteCaptured,
    ExecuteInteractive
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub action: CommandAction,
    pub is_interactive: bool,
}

pub fn is_protected(name: &str) -> bool {
    let protected = ["reg", "del", "clear", "help", "exit"];
    protected.contains(&name.to_lowercase().as_str())
}

impl Command {
    pub fn new(name: &str, description: &str, action: CommandAction) -> Self {
        let name = name.to_string();
        let description = description.to_string();

        Self { 
            name, 
            description,
            action,
            is_interactive: false
        }
    }
}

struct TreeNode {
    pub command: Command,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub struct CommandTree {
    root: Option<Box<TreeNode>>,
}

impl CommandTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, command: Command) {
        self.root = Self::insert_recursive(self.root.take(), command);
    }

    fn insert_recursive(node: Option<Box<TreeNode>>, command: Command) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                command,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if command.name < n.command.name {
                    n.left = Self::insert_recursive(n.left.take(), command);
                } else if command.name > n.command.name {
                    n.right = Self::insert_recursive(n.right.take(), command);
                }
                Some(n)
            }
        }
    }

    pub fn get_all_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        Self::traverse(&self.root, &mut names);
        names
    }

    fn traverse(node: &Option<Box<TreeNode>>, names: &mut Vec<String>) {
        if let Some(n) = node {
            Self::traverse(&n.left, names);
            names.push(n.command.name.clone());
            Self::traverse(&n.right, names);
        }
    }

    pub fn search(&self, name: String) -> Option<&Command> {
        let mut current = &self.root;
        let mut name = name;
        if name.starts_with('!') {
            name.pop();
        }
        while let Some(node) = current {
            if name == node.command.name {
                return Some(&node.command);
            } else if name < node.command.name {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        None
    }

    pub fn find_prediction(&self, prefix: &str) -> Option<String> {
        if prefix.is_empty() { return None; }
        self.search_prefix(&self.root, prefix)
    }

    fn search_prefix(&self, node: &Option<Box<TreeNode>>, prefix: &str) -> Option<String> {
        match node {
            Some(n) => {
                if n.command.name.starts_with(prefix) {
                    Some(n.command.name.clone())
                } else if prefix < &n.command.name {
                    self.search_prefix(&n.left, prefix)
                } else {
                    self.search_prefix(&n.right, prefix)
                }
            }
            None => None,
        }
    }
    pub fn delete(&mut self, name: &str) {
        self.root = Self::delete_recursive(self.root.take(), name);
    }

    fn delete_recursive(node: Option<Box<TreeNode>>, name: &str) -> Option<Box<TreeNode>> {
        let mut n = node?;

        if name < n.command.name.as_str() {
            n.left = Self::delete_recursive(n.left.take(), name);
            Some(n)
        } else if name > n.command.name.as_str() {
            n.right = Self::delete_recursive(n.right.take(), name);
            Some(n)
        } else {
            if n.left.is_none() { return n.right; }
            if n.right.is_none() { return n.left; }

            let successor_command = Self::find_min(&n.right);
            n.command = successor_command.clone();
            n.right = Self::delete_recursive(n.right.take(), &successor_command.name);
            Some(n)
        }
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> Command {
        let mut current = node.as_ref().unwrap();
        while let Some(ref left) = current.left {
            current = left;
        }
        current.command.clone()
    }
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut commands = Vec::new();
        self.collect_all(&self.root, &mut commands);
        
        let json = serde_json::to_string_pretty(&commands);

        std::fs::write(filename, json.unwrap())?;
        Ok(())
    }

    pub fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let data = std::fs::read_to_string(filename)?;
        
        let commands: Vec<Command> = serde_json::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        for cmd in commands {
            self.insert(cmd);
        }
        
        Ok(())
    }

    fn collect_all(&self, node: &Option<Box<TreeNode>>, list: &mut Vec<Command>) {
        if let Some(n) = node {
            self.collect_all(&n.left, list);
            list.push(n.command.clone());
            self.collect_all(&n.right, list);
        }
    }
}
