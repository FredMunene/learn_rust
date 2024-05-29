// create area of triangle
// triangle has a height and weight
// use structs

pub struct Triangle {
    height : i32,
    width : i32,
}

impl Triangle {

    pub fn new(height : i32, width :i32) -> Self{
        return Triangle{height,width};
    }
    pub fn area (&self) -> i32 {
        return self.height* self.width;
    }
}

