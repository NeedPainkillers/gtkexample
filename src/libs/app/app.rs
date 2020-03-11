use gtk;
use crate::libs::model::student::StudentList;

pub struct app
{
    window: gtk::Window,
    storage: StudentList
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