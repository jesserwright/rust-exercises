#[allow(dead_code)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64, // should be an option?
}

#[allow(dead_code)]
impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_list() {
        let mut averaged_collection = AveragedCollection::new();
        averaged_collection.add(32);
        assert_eq!(averaged_collection.average(), 32.0);
    }

    #[test]
    fn test_remove_from_list() {
        let mut averaged_collection = AveragedCollection::new();
        averaged_collection.add(32);
        averaged_collection.remove();
        assert_eq!(averaged_collection.average(), 0.0);
    }
}
