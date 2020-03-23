use std::string;
use std::collections::LinkedList;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use itertools::Itertools;
use itertools::rciter;

use crate::libs::app::app::app;
use std::borrow::{BorrowMut, Borrow};
use std::fs::File;


#[derive(Clone)]
pub struct Student
{
    pub id: String,
    pub name: String,
    pub age: u8,
    pub _id_internal: i32
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Student: id: \"{0}\", name: \"{1}\", age: {2};", self.id, self.name, self.age as char)
    }
}


#[derive(Clone)]
pub struct RefStudent
{
    pub item: RefCell<Student>
}

impl fmt::Display for RefStudent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Student: id: \"{0}\", name: \"{1}\", age: {2};", self.item.borrow().id, self.item.borrow().name, self.item.borrow().age as char)
    }
}

impl From<RefCell<Student>> for RefStudent {
    fn from(item: RefCell<Student>) -> Self {
        RefStudent {item: item.clone()}
    }
}

impl RefStudent
{
    pub fn new(item: Student) -> RefStudent
    {
        RefStudent {item: RefCell::new(item)}
    }
}


pub struct RcRefStudent
{
    pub item: Rc<RefStudent>
}

impl Clone for RcRefStudent
{
    fn clone(&self) -> RcRefStudent
    {
        RcRefStudent {item: self.item.clone()}
    }
}

impl From<Rc<RefStudent>> for RcRefStudent
{
    fn from(item: Rc<RefStudent>) -> Self {
        RcRefStudent {item: item.clone()}
    }
}

impl fmt::Display for RcRefStudent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.item.item.borrow())
    }
}

impl RcRefStudent
{
    pub fn new(item: Student) -> RcRefStudent
    {
        RcRefStudent {item: Rc::new(RefStudent::new(item))}
    }

    pub fn borrow_mut(&mut self) -> std::cell::RefMut<Student>
    {
        return self.item.item.borrow_mut();
    }
}


#[derive(Clone)]
pub struct StudentList
{
    storage: LinkedList<Rc<RefCell<Student>>>
}

impl StudentList
{
    pub fn new() -> StudentList
    {
        StudentList
            {
                storage: LinkedList::new()
            }
    }

    pub fn add(&mut self, item: Rc<RefCell<Student>>) -> &mut StudentList
    {
        self.storage.push_back(item);
        return self;
    }

    pub fn delete(&mut self, index: i32) -> &mut StudentList
    {
        //TODO: pop until index is equal to given, then push back popped vales except deleted one
        self.storage = rciter(self.storage.clone()).filter(|x|
            RefCell::borrow(x.as_ref())._id_internal != index ).collect();
        return self;
    }

    pub fn write_to_file(&mut self, path: &String) -> &mut StudentList
    {
        let mut file = std::fs::File::create(path).expect(format!("{0}{1}{2}","Cannot create file at '", path, "'").as_str());
        let mut writer = BufWriter::new(file);

        //TODO:iter
        let mut text = String::new();
        for i in self.storage.clone()
            {
                //println!("{}", i.as_ref().borrow());
                text.push_str(format!("{}\n",i.as_ref().borrow()).as_str());
            }
        writer.write_all(text.as_bytes()).expect((format!("{0}{1}{2}", "Cannot write to file '", path, "'").as_str()));

        return self;
    }
    pub fn write_to_file_choosen(&mut self, path: &std::path::Path) -> &mut StudentList
    {
        let mut file = std::fs::File::create(path).expect(format!("{0}{1}{2}","Cannot create file at '", (*path).display(), "'").as_str());
        let mut writer = BufWriter::new(file);

        //TODO:iter
        let mut text = String::new();
        for i in self.storage.clone()
            {
                //println!("{}", i.as_ref().borrow());
                text.push_str(format!("{}\n",i.as_ref().borrow()).as_str());
            }
        writer.write_all(text.as_bytes()).expect((format!("{0}", "Cannot write to file").as_str()));

        return self;
    }
}