use self::input::INPUT;
use anyhow::{anyhow, Result};
use std::{
    cell::RefCell, collections::HashMap, num::ParseIntError, rc::Rc, str::FromStr, time::Duration,
};

mod input;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

impl FromStr for File {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let size = tokens.next().unwrap().parse::<usize>()?;
        let name = tokens.next().unwrap().to_owned();
        Ok(Self { name, size })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Folder {
    name: String,
    items: HashMap<String, Item>,
    size: Option<usize>,
    parent: Option<Rc<RefCell<Folder>>>,
}

impl FromStr for Folder {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let label = tokens.next().unwrap();
        if label != "dir" {
            Err(anyhow!("String is not labelled as a directory"))
        } else {
            let name = tokens.next().unwrap().to_owned();
            Ok(Self::new(name, None))
        }
    }
}

impl Folder {
    fn new(name: String, parent: Option<Rc<RefCell<Folder>>>) -> Self {
        Self {
            name,
            items: HashMap::new(),
            size: None,
            parent,
        }
    }
}

#[derive(Clone, Debug)]
enum Item {
    Folder(Rc<RefCell<Folder>>),
    File(File),
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(file) = s.parse::<File>() {
            Ok(Self::File(file))
        } else {
            Ok(Self::Folder(Rc::new(RefCell::new(s.parse::<Folder>()?))))
        }
    }
}

impl Item {
    fn get_name(&self) -> String {
        match self {
            Item::Folder(folder) => folder.borrow().name.to_owned(),
            Item::File(file) => file.name.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
enum ChangeDirectoryTarget {
    In(String),
    Out,
    Root,
}

impl FromStr for ChangeDirectoryTarget {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ".." => Self::Out,
            "/" => Self::Root,
            sub_folder => Self::In(sub_folder.to_owned()),
        })
    }
}

#[derive(Clone, Debug)]
enum Command {
    ChangeDirectory(ChangeDirectoryTarget),
    List,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let prompt = tokens.next().unwrap();
        if prompt != "$" {
            Err(anyhow!("No command prompt"))
        } else {
            let command = tokens.next().unwrap();
            match command {
                "cd" => Ok(Self::ChangeDirectory(
                    tokens.next().unwrap().parse::<ChangeDirectoryTarget>()?,
                )),
                "ls" => Ok(Self::List),
                unrecognized_command => {
                    Err(anyhow!("Unrecognized command: {unrecognized_command}"))
                }
            }
        }
    }
}

pub(crate) const ROOT: &str = "/";

fn deduce_file_system_structure(input: &str) -> Rc<RefCell<Folder>> {
    let root = Rc::new(RefCell::new(Folder::new(ROOT.to_owned(), None)));
    let mut current_folder = root.clone();
    for line in input.lines() {
        if let Ok(command) = line.parse::<Command>() {
            // println!("{line} -> {command:?}");
            match command {
                Command::ChangeDirectory(target) => match target {
                    ChangeDirectoryTarget::In(sub_folder) => {
                        let s;
                        if let Item::Folder(new_folder) = (*current_folder)
                            .borrow()
                            .items
                            .iter()
                            .find(|(_, item)| item.get_name() == sub_folder)
                            .unwrap()
                            .1
                        {
                            s = new_folder.clone();
                        } else {
                            panic!();
                        }
                        current_folder = s;
                    }
                    ChangeDirectoryTarget::Out => {
                        let parent = (*current_folder).borrow().parent.as_ref().unwrap().clone();
                        current_folder = parent;
                    }
                    ChangeDirectoryTarget::Root => current_folder = root.clone(),
                },
                Command::List => {}
            }
        } else if let Ok(item) = line.parse::<Item>() {
            // println!("    {line} -> {item:?}");
            (*current_folder)
                .borrow_mut()
                .items
                .insert(item.get_name().to_owned(), item.clone());

            if let Item::Folder(sub_folder) = item {
                (*sub_folder).borrow_mut().parent = Some(current_folder.clone());
            }
        }
    }
    root
}

fn inject_folder_sizes(folder: Rc<RefCell<Folder>>) -> usize {
    // println!("inject_folder_sizes for {}", (*folder).borrow().name);
    let b_size = folder.borrow().size;
    if let Some(size) = b_size {
        size
    } else {
        let size = (*folder)
            .borrow()
            .items
            .iter()
            .fold(0, |current, (_, item)| {
                current
                    + match item {
                        Item::Folder(sub_folder) => inject_folder_sizes(sub_folder.clone()),
                        Item::File(file) => file.size,
                    }
            });
        println!(
            "computed size {size} for folder {}",
            (*folder).borrow().name
        );
        (*folder).borrow_mut().size = Some(size);
        size
    }
}

pub(crate) fn deduce_file_system(input: &str) -> Rc<RefCell<Folder>> {
    let root = deduce_file_system_structure(input);
    inject_folder_sizes(root.clone());
    root
}

#[cfg(test)]
pub(crate) fn find_folder(root: Rc<RefCell<Folder>>, name: &str) -> Option<Rc<RefCell<Folder>>> {
    if root.borrow().name == name {
        Some(root)
    } else {
        root.borrow().items.iter().find_map(|(_, item)| match item {
            Item::Folder(sub_folder) => {
                if sub_folder.borrow().name == name {
                    Some(sub_folder.clone())
                } else {
                    find_folder(sub_folder.clone(), name)
                }
            }
            _ => None,
        })
    }
}

fn collect_folders(
    folder: Rc<RefCell<Folder>>,
    predicate: &dyn Fn(&Folder) -> bool,
    folders: &mut Vec<Rc<RefCell<Folder>>>,
) {
    if predicate(&(*folder).borrow()) {
        folders.push(folder.clone());
    }
    (*folder).borrow().items.iter().for_each(|(_, item)| {
        if let Item::Folder(sub_folder) = item {
            collect_folders(sub_folder.clone(), predicate, folders)
        }
    });
}

fn solve_for(input: &str) -> (usize, usize) {
    let root = deduce_file_system(input);
    let mut small_folders = vec![];
    collect_folders(
        root.clone(),
        &|folder: &Folder| folder.size.unwrap() <= 100000,
        &mut small_folders,
    );
    let part1 = small_folders.iter().fold(0, |current, folder| {
        current + (*folder).borrow().size.unwrap()
    });

    let unused_space = 70_000_000 - (*root).borrow().size.unwrap();
    let need_to_free = 30_000_000 - unused_space;
    let mut large_enough_folders = vec![];
    collect_folders(
        root,
        &|folder: &Folder| folder.size.unwrap() >= need_to_free,
        &mut large_enough_folders,
    );
    let best_folder = large_enough_folders
        .iter()
        .min_by_key(|folder| (*folder).borrow().size.unwrap())
        .unwrap();
    let part2 = (*best_folder).borrow().size.unwrap();
    
    (part1, part2)
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let (part1, part2) = solve_for(INPUT);
    (part1, part2, Duration::new(0, 0))
}
