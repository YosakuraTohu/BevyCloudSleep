use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Map2D {
    x_axis: HashMap<i32, Box<Vec<(i32, i32)>>>,
    y_axis: HashMap<i32, Box<Vec<(i32, i32)>>>,
    max_len_of_x: i32,
    max_len_of_y: i32,
}

impl Map2D {
    pub fn new() -> Self {
        Map2D {
            x_axis: HashMap::new(),
            y_axis: HashMap::new(),
            max_len_of_x: 0,
            max_len_of_y: 0,
        }
    }

    pub fn insert(&mut self, (from_x, from_y, to_x, to_y): (i32, i32, i32, i32)) {
        if to_x - from_x > self.max_len_of_x {
            self.max_len_of_x = to_x - from_x;
        }
        if from_y - to_y > self.max_len_of_x {
            self.max_len_of_y = from_y - to_y;
        }

        if !self.x_axis.contains_key(&from_x) {
            self.x_axis.insert(from_x, Box::new(Vec::new()));
        }
        if !self.x_axis.contains_key(&to_x) {
            self.x_axis.insert(to_x, Box::new(Vec::new()));
        }
        if !self.y_axis.contains_key(&from_y) {
            self.y_axis.insert(from_y, Box::new(Vec::new()));
        }
        if !self.y_axis.contains_key(&to_y) {
            self.y_axis.insert(to_y, Box::new(Vec::new()));
        }

        if let Some(line) = self.x_axis.get_mut(&from_x) {
            line.insert(line.len(), (from_y, to_y));
        }
        if let Some(line) = self.x_axis.get_mut(&to_x) {
            line.insert(line.len(), (from_y, to_y));
        }
        if let Some(line) = self.y_axis.get_mut(&from_y) {
            line.insert(line.len(), (from_x, to_x));
        }
        if let Some(line) = self.y_axis.get_mut(&to_y) {
            line.insert(line.len(), (from_x, to_x));
        }
    }

    #[allow(dead_code)]
    pub fn is_on_edge(&self, (&x_pos, &y_pos): (&i32, &i32)) -> bool {
        for d in x_pos - 16..x_pos + 16 {
            if let Some(b) = self.x_axis.get(&d) {
                for line in b.iter() {
                    if line.0 >= y_pos && line.1 <= y_pos {
                        return true;
                    }
                }
            }
        }

        for d in y_pos - 16..y_pos + 16 {
            if let Some(b) = self.y_axis.get(&d) {
                for line in b.iter() {
                    if line.0 <= x_pos && line.1 >= x_pos {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_in_reat(&self, (&x_pos, &y_pos): (&i32, &i32)) -> bool {
        for d in x_pos - self.max_len_of_x..x_pos + self.max_len_of_x {
            if let Some(b) = self.x_axis.get(&d) {
                for line in b.iter() {
                    if line.0 >= y_pos && line.1 <= y_pos {
                        if let Some(b) = self.y_axis.get(&line.1) {
                            for line in b.iter() {
                                if line.0 <= x_pos && line.1 >= x_pos {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        for d in y_pos - self.max_len_of_y..y_pos + self.max_len_of_y {
            if let Some(b) = self.y_axis.get(&d) {
                for line in b.iter() {
                    if line.0 <= x_pos && line.1 >= x_pos {
                        if let Some(b) = self.x_axis.get(&line.0) {
                            for line in b.iter() {
                                if line.0 >= y_pos && line.1 <= y_pos {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }
}
