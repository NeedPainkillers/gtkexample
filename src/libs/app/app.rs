use gtk;
use crate::libs::model::student::StudentList;

#[derive(Clone)]
pub struct app
{
    pub window: gtk::Window,
    pub storage: StudentList
}

impl app
{
    pub fn new(window: gtk::Window) -> app
    {
        app
            {
                window,
                storage: StudentList::new()
            }
    }
}