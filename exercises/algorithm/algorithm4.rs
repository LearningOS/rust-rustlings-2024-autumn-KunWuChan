/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
use std::thread::sleep_ms;

/*
二叉搜索树：
    每个节点的左子树所有值都小于该节点的值。
    每个节点的右子树所有值都大于该节点的值。
    左右子树也必须是二叉搜索树。
*/ 

#[derive(Debug)]
struct TreeNode<T> //树节点
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>, //使用Box管理堆内存
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    // 如果为空，则创建，否则找到value的区间【左节点，右节点】 插入
    fn insert(&mut self, value: T) {
        match self.root { //root节点是否存在
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut node) => node.insert(value),
        }
        
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {

        self.root.as_ref().map_or(false, |node| node.search(&value))
        //---- ^^^^^^^^^ expected `Box<TreeNode<_>>`, found `&Box<TreeNode<T>>`
        // match self.root.as_ref() {
        //     None => false,
        //     Some(node) => {
        //         match value.cmp( &node.value){
        //             Ordering::Equal => true,
        //             /*
        //                 as_ref 借用一下所有权，不改代码
        //                 None时返回false；如果 Option 是 Some(value)，将 value 传入闭包 f 并返回闭包的结果。
        //                 闭包|a|中的a是节点的引用，复制子树节点再查找
        //             */ 
        //             Ordering::Less => node.left.as_ref().map_or(
        //                 false, |n|BinarySearchTree { root: Some(*n.clone()) }.search(value)),
        //             Ordering::Greater => node.right.as_ref().map_or(
        //                 false, |n|BinarySearchTree{ root:Some(*n.clone())}.search(value)),
        //         }
        //     }
        // }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {} //不重复插入
            Ordering::Less => {//小于，左子树
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(value))); // Option  Box TreeNode
                } else {
                    self.left.as_mut().unwrap().insert(value); //因为要修改左子树
                }
            }
            Ordering::Greater => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(value))); // Option  Box TreeNode
                } else {
                    self.right.as_mut().unwrap().insert(value);
                }
            }
        }
    }

    fn search(&self, value: &T) -> bool  {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => self.left.as_ref().map_or(false, |node| node.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |node| node.search(value)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


