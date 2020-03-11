use std::string;
use std::collections::LinkedList;
use crate::libs::app::app::app;

pub struct Student
{
    id: String,
    name: String,
    age: u8
}

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

    fn add()
    {

    }
}