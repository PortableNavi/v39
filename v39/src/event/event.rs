#[derive(Debug, Clone)]
pub enum EventData
{
    Str(String),
    Int(isize),
    Uint(usize),
    Float(f32),
    Double(f64),
    Bool(bool),
}


#[derive(Clone, Debug)]
pub struct Event
{
    pub id: u32,
    pub data: Vec<EventData>,
}


impl Event
{
    pub fn new(id: impl Into<u32>, data: Vec<EventData>) -> Self
    {
        Self {data, id: id.into()}
    }
}


