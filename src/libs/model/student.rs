use std::string;
use std::collections::LinkedList;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fmt;

use itertools::Itertools;

use crate::libs::app::app::app;

#[derive(Clone)]
pub struct Student
{
    pub id: String,
    pub name: String,
    pub age: u8
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Student: id: \"{0}\", name: \"{1}\", age: {2};", self.id, self.name, self.age as char)
    }
}

#[derive(Clone)]
pub struct StudentList
{
    storage: LinkedList<Student>
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

    pub fn add(&mut self, item: Student) -> &mut StudentList
    {
        self.storage.push_back(item);
        return self;
    }

    pub fn delete(&mut self, index: i32) -> &mut StudentList
    {
        //TODO: pop until index is equal to given, then push back popped vales except deleted one
        return self;
    }

    pub fn write_to_file(&mut self, path: &String) -> &mut StudentList
    {
        let mut file = std::fs::File::create(path).expect(format!("{0}{1}{2}","Cannot create file at '", path, "'").as_str());
        let mut writer = BufWriter::new(file);
        let text = self.storage.iter().join("\n");
        writer.write_all(text.as_bytes()).expect((format!("{0}{1}{2}", "Cannot write to file '", path, "'").as_str()));
        return self;
    }
}