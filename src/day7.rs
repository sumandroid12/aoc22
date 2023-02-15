use std::{cell::RefCell, rc::Rc};
enum Item {
    Dir {
        name: String,
        items: Vec<Rc<RefCell<Item>>>,
    },
    File {
        name: String,
        size: u32,
    },
}

#[test]
fn size_up() {
    let input = include_str!("day7.in");
    let root = Rc::new(RefCell::new(Item::Dir {
        name: '/'.to_string(),
        items: vec![],
    }));
    let mut cur_dir = root.clone();
    let mut stack = vec![];
    stack.push(cur_dir.clone());
    let mut lines = input.lines();
    while let Some(op) = lines.next() {
        let mut c = op.split(' ');
        let fr = c.next();
        let sc = c.next();
        let th = c.next();
        match (fr, sc, th) {
            (Some("$"), Some("cd"), Some("/")) => {
                cur_dir = root.clone();
                stack.clear();
            }
            (Some("$"), Some("cd"), Some("..")) => {
                cur_dir = stack.pop().unwrap();
            }
            (Some("$"), Some("cd"), Some(dir)) => {
                let mut temp_cur_dir = cur_dir.clone();
                if let Item::Dir {
                    name: _,
                    ref mut items,
                } = *cur_dir.borrow_mut()
                {
                    let into_dir = items.iter().find(|x| {
                        if let Item::Dir { ref name, items: _ } = *x.borrow() {
                            name == dir
                        } else {
                            false
                        }
                    });
                    temp_cur_dir = into_dir.unwrap().clone();
                    stack.push(cur_dir.clone());
                }
                cur_dir = temp_cur_dir;
            }
            (Some("$"), Some("ls"), None) => {}
            (Some("dir"), Some(dir_name), None) => {
                if let Item::Dir {
                    name: _,
                    ref mut items,
                } = *cur_dir.borrow_mut()
                {
                    items.push(Rc::new(RefCell::new(Item::Dir {
                        name: dir_name.to_string(),
                        items: vec![],
                    })))
                }
            }
            (Some(size), Some(file_name), None) => {
                if let Item::Dir {
                    name: _,
                    ref mut items,
                } = *cur_dir.borrow_mut()
                {
                    items.push(Rc::new(RefCell::new(Item::File {
                        name: file_name.to_string(),
                        size: size.parse().unwrap(),
                    })))
                }
            }
            _ => panic!(),
        }
    }
    let mut size_list = vec![];
    let root_size = dir_size(root.clone(), &mut size_list);
    let total_sz = 70000000;
    let reqd_space = 30000000 - (total_sz - root_size);
    size_list.sort_unstable();
    let freed_space = size_list
        .iter()
        .filter(|s| **s >= reqd_space)
        .nth(0)
        .unwrap();
    println!("{}", *freed_space);
}

fn dir_size(root: Rc<RefCell<Item>>, size_list: &mut Vec<u32>) -> u32 {
    let mut sum = 0;
    if let Item::Dir { name: _, ref items } = *root.borrow() {
        sum = items
            .iter()
            .map(|itm| match *itm.borrow() {
                Item::Dir { name: _, items: _ } => {
                    let size = dir_size(itm.clone(), size_list);
                    size_list.push(size);
                    size
                }
                Item::File { name: _, ref size } => *size,
            })
            .sum::<u32>();
    }
    return sum;
}
