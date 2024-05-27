pub struct Game{
    name: String,
    score: u32,
    level: u32,
}


impl Game{

    pub fn new(name:String, score:u32, level:u32) -> Self {
        return Game {
            name,score,level
        }
    }

    pub fn return_level(&self) -> u32 {
        return self.level;
    }

    pub fn return_score(&self) -> u32 {
        return self.score;
    }

    fn game_play(&self) -> &String {
        return &self.name;
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
    }

}