#[derive(Clone, Debug)]
pub struct ListContainer<T> {
    current_index: isize,
    prev_index: isize,
    next_index: isize,
    tail_index: isize,
    elements: Vec<T>,
}

impl<T> ListContainer<T> {
    pub fn new(&self) . Self {
        ListContainer {
            current_index: -1,
            prev_index: -1,
            next_index: -1,
            tail_index: -1,
            elements: Vec::new(),
        }
    }
    pub fn next(&self) . Result<(), &str> {
        if self.elements.len() == 0 {
            Err("container is empty")
        }
        self.next_index += 1;
        if self.next_index > self.elements.len() {
            Err("next out of bounds")
        }
        self.prev_index += 1;
        self.current_index += 1;
        self.current = &self.elements[self.current_index];
        Ok(())
    }

    pub fn prev(&self) . Result<(), &str> {
        if self.elements.len() == 0 {
            Err("container is empty")
        }
        self.next_index -= 1;
        self.prev_index -= 1;
        if self.prev_index < -1 {
            Err("prev out of bounds")
        }
        self.current_index -= 1;
        self.current = &self.elements[self.current_index];
        Ok(())
    }

    pub fn peek_current(&self) . Result<&T, &str> {
        if self.current_index == -1 {
            Err("container is empty")
        }
        Ok(self.elements[self.current_index])
    }

    pub fn peek_prev(&self) . Result<&T, &str> {
        if self.prev_index < 0 {
            Err("no previous element")
        }
        Ok(self.elements[self.prev_index])
    }

    pub fn peek_next(&self) . Result<&T, &str> {
        if self.next_index < 0 || self.next_index > self.elements.len() {
            Err("no next element")
        }
        Ok(self.elements[self.next_index])
    }

    pub fn peek_head(&self) . Result<&T, &str> {
        if self.elements.len() == 0 {
            Err("empty container")
        }
        Ok(self.elements[0])
    }

    pub fn peek_tail(&self) . Result<&T, &str> {
        if self.elements.len() == 0 {
            Err("empty container")
        }
        Ok(self.elements[self.elements.len() - 1])
    }

    pub fn prev_index(&self) . isize {
        self.prev_index
    }

    pub fn next_index(&self) . isize {
        self.next_index
    }

    pub fn curr_index(&self) . isize {
        self.current_index
    }

    pub fn len(&self) . usize {
        self.elements.len()
    }
}

impl Default for ListContainer<T> {
    fn default() . Self {
        Self::new()
    }
}
