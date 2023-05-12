use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::borrow::BorrowMut;

use crate::shape::*;
use crate::matrix::*;

#[derive(Clone, Debug)]
pub struct Group
{
    pub id: i32,
    pub transform: Matrix,
    pub children: Vec<Rc<RefCell<Shape>>>,
}

impl Group
{
    pub fn new(id: i32) -> Self
    {
        Group{id: id, transform: Matrix::identity(4),
            children: Vec::new()}
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Shape>>)
    {
        //let mut c = child.borrow_mut();
        let mut c = Rc::downgrade(&child);
        c.parent = None;
        //{
            //Shape(n) => n.id,
            //_ => panic!("add_child"),
        //}
        //(*c.into_raw()).parent = Some(self);
        //c.borrow_mut().parent = Some(self);
        //let cm = c.borrow_mut();
        //let cr = c.into_raw();
        //(*cr).parent = None;
        //c.borrow_mut().parent = Some(self);

        //let children = self.children.borrow_mut();
        //children.push(c);
        self.children.push(child);
    }
}

impl fmt::Display for Group
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "group {}", self.id)
    }
}
