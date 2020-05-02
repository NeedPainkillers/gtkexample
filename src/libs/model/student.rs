use std::collections::LinkedList;
use std::io::{BufWriter, BufReader};
use std::io::prelude::*;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use itertools::rciter;

use regex::Regex;

/// Data model
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

/// Logic model
#[derive(Clone)]
pub struct StudentList
{
    pub storage: LinkedList<Rc<RefCell<Student>>>,
    pub _count: i32
}

impl StudentList
{
    pub fn new() -> StudentList
    {
        StudentList
            {
                storage: LinkedList::new(),
                _count: 0
            }
    }

    pub fn add(&mut self, item: Rc<RefCell<Student>>) -> &mut StudentList
    {
        self.storage.push_back(item);
        return self;
    }

    pub fn delete(&mut self, index: i32) -> &mut StudentList
    {
        self.storage = rciter(self.storage.clone()).filter(|x|
            RefCell::borrow(x.as_ref())._id_internal != index ).collect();
        return self;
    }

    pub fn read_from_file(&mut self, path: &std::path::Path) -> &mut StudentList
    {
        let mut file = std::fs::File::open(path).expect(format!("{0}{1}{2}","Cannot open file at '", (*path).display(), "'").as_str());
        let mut reader = BufReader::new(file);

        let mut text = String::new();
        let mut res = reader.read_to_string(&mut text);

        // pattern
        // ((?<type>.*):\s*id:\s*"(?<id>.*)",\s*name:\s*"(?<name>.*)",\s*age:\s*(?<age>.).*;)
        let reg = Regex::new("((?P<type>.*):\\s*id:\\s*\"(?P<id>.*)\",\\s*name:\\s*\"(?P<name>.*)\",\\s*age:\\s*(?P<age>.).*;)").unwrap();
        let mut i = -1;
        let captures: LinkedList<Rc<RefCell<Student>>> = reg.captures_iter(text.as_str())
            .map(|c| {
                i+=1;
                Rc::new(RefCell::new(Student {
                    name: c.name("name").unwrap().as_str().to_string(),
                    id: c.name("id").unwrap().as_str().to_string(),
                    age: c.name("age").unwrap().as_str().as_bytes()[0],
                    _id_internal: i }))
            })
            .collect();

        self.storage.clear();
        self.storage = captures;
        self._count = i;

        return self;
    }

    pub fn write_to_file(&mut self, path: &String) -> &mut StudentList
    {
        let mut file = std::fs::File::create(path).expect(format!("{0}{1}{2}","Cannot create file at '", path, "'").as_str());
        let mut writer = BufWriter::new(file);

        let mut text = String::new();
        for i in self.storage.clone()
            {
                //println!("{}", i.as_ref().borrow());
                text.push_str(format!("{}\n",i.as_ref().borrow()).as_str());
            }
        writer.write_all(text.as_bytes()).expect(format!("{0}{1}{2}", "Cannot write to file '", path, "'").as_str());

        return self;
    }

    pub fn write_to_file_choosen(&mut self, path: &std::path::Path) -> &mut StudentList
    {
        let mut file = std::fs::File::create(path).expect(format!("{0}{1}{2}","Cannot create file at '", (*path).display(), "'").as_str());
        let mut writer = BufWriter::new(file);

        let mut text = String::new();
        for i in self.storage.clone()
            {
                //println!("{}", i.as_ref().borrow());
                text.push_str(format!("{}\n",i.as_ref().borrow()).as_str());
            }
        writer.write_all(text.as_bytes()).expect(format!("{0}", "Cannot write to file").as_str());

        return self;
    }
}