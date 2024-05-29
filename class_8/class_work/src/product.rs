// pub struct Book {
//     pub name : String,
//     pub  status : bool,
//    pub  category : String,
//  }


pub struct Book {

    name : String,
    status : bool,
    category : String,
}

impl Book {
    
    pub fn new(name : String,status : bool,category : String,) -> Self{
        return Book{name,status,category}

    }
    pub fn avail (&self) -> bool {
        return self.status
    }

    pub fn categ (&self) {
        println!("{}",&self.category)
    }
}