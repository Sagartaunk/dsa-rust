struct Hash {
    map: Vec<Option<Vec<(String, i32)>>>,
}
impl Hash{
    fn new() -> Self{
        Self{map:vec![None; 20]}
    }
    fn is_full(&self) -> bool {
        let count: usize = self.map.iter().map(|slot| slot.as_ref().map_or(0, |v| v.len())).sum();
        count as f64 / self.map.len() as f64 >= 0.75
    }
    fn add(&mut self , key: String , val: i32){
        let index = hash(key.trim()) % self.map.len();
        if self.is_full(){
            self.extend();
        }
        if let Some(vec) = self.map[index].as_mut() {
            for (k, v) in vec.iter_mut() {
                if k == &key {
                    *v = val;
                    return;
                }
            }
        }
        if self.map[index].is_none(){
            self.map[index] = Some(vec![(key.clone() , val)]);
            return;
        }
        self.map[index].as_mut().unwrap().push((key,val));
    }
    fn lookup(&self , key:String) ->Option<i32> {
        let index = hash(key.trim()) % self.map.len();
        if let Some(bucket) = self.map[index].as_ref() {
            for (k, v) in bucket.iter() {
                if k == key.trim() {
                    return Some(*v);
                }
            }
        }
        None
    }
    fn extend(&mut self){
        let mut new_map : Vec<Option<Vec<(String, i32)>>> = vec![None ; 2*self.map.len()];
        for bucket in &self.map {
            if let Some(vec) = bucket {
                for (key , val) in vec{
                    let index = hash(key) % new_map.len();
                    if new_map[index].is_none() {
                        new_map[index] = Some(vec![(key.to_string(),*val)]);
                    }
                    else {
                        new_map[index].as_mut().unwrap().push((key.to_string(), *val));
                    }
                }
            }
        }
        self.map = new_map;
    }
    fn delete(&mut self, key: String) {
        let index = hash(key.trim()) % self.map.len();
        if let Some(bucket) = self.map[index].as_mut() {
            if let Some(pos) = bucket.iter().position(|(k, _)| k == &key) {
                bucket.remove(pos);
            }
        }
    }

}
fn hash(key: &str) -> usize{
    let mut hash: usize = 0;
    for i in key.chars() {
        hash = hash * 31 + i as usize;
    }
    hash
}
fn main() {}
