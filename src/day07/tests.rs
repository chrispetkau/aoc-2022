use super::{input, solve_for};
use crate::day07::{deduce_file_system, find_folder, ROOT};

const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[test]
fn folder_sizes() {
    let root = deduce_file_system(INPUT);
    assert_eq!(584, (*find_folder(root.clone(), "e").unwrap()).borrow().size.unwrap());
    assert_eq!(94853, (*find_folder(root.clone(), "a").unwrap()).borrow().size.unwrap());
    assert_eq!(24933642, (*find_folder(root.clone(), "d").unwrap()).borrow().size.unwrap());
    assert_eq!(48381165, (*find_folder(root, ROOT).unwrap()).borrow().size.unwrap());
}

#[test]
fn part1() {
    assert_eq!(95437, solve_for(INPUT).0);
    assert_eq!(1297159, solve_for(input::INPUT).0);
}

#[test]
fn part2() {
    assert_eq!(24933642, solve_for(INPUT).1);
    assert_eq!(3866390, solve_for(input::INPUT).1);
}
