fn main() {
    binary_tree();
    test_matching();
}

use std::cmp::Ordering::{self, *};

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

#[test]
fn test_compare() {
    assert_eq!(compare(1, 2), Less);
    assert_eq!(compare(2, 1), Greater);
    assert_eq!(compare(1, 1), Equal);
}

enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
}

#[test]
fn test_size_of() {
    use std::mem::size_of;

    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);
}

use std::collections::HashMap;

enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

fn binary_tree() {
    let mut mars_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    mars_tree.add("Jupiter");
    mars_tree.add("Mercury");

    let mut uranus_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    uranus_tree.add("Venus");

    let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));

    match tree {
        BinaryTree::Empty => println!("empty"),
        BinaryTree::NonEmpty(node) => println!("non-empty: {}", node.element),
    }
}

fn test_matching() {
    let n = 1u32;
    match n {
        0 | 1 => println!("small"),
        2 | 3 => println!("medium"),
        _ => println!("large"),
    }

    let pair = (0, -2);

    match pair {
        p @ (0, 0) => println!("origin: {:?}", p),
        p @ (0, y) => println!("x axis: {:?}", p),
        p @ (x, 0) => println!("y axis: {:?}", p),
        p @ (x, y) => println!("other: {:?}", p),
    }
}
