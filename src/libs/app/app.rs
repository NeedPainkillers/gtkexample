use gtk;
use crate::libs::model::student::StudentList;

#[derive(Clone)]
pub struct App
{
    pub window: gtk::Window,
    pub storage: StudentList,
    pub idx: i32
}

impl App
{
    pub fn new(window: gtk::Window) -> App
    {
        App
            {
                window,
                storage: StudentList::new(),
                idx: 0
            }
    }
}