// https://stackoverflow.com/a/63410264
// Modified to fit our use case
#[derive(Debug)]
pub struct Stack<'a, T> {
    pub element: Option<&'a T>,
    tail: Option<&'a Stack<'a, T>>,
}

impl<'a, T> Stack<'a, T> {
    pub fn new(element: &'a T) -> Self { Stack { element: Some(element), tail: None } }

    pub fn pop(&mut self) -> Option<&T> {
        let ret = self.element;
        if self.tail.is_some() {
            self.element = self.tail.unwrap().element;
            self.tail = self.tail.unwrap().tail;
        } else {
            self.element = None
        }
        ret
    }

    pub fn _get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            self.element
        } else {
            self.tail.and_then(|tail| tail._get(index - 1))
        }
    }

    pub fn tail(&self) -> Option<&'a Stack<'a, T>> { self.tail }

    pub fn push<'b>(&'b self, element: &'b T) -> Stack<'b, T> { Stack { element: Some(element), tail: Some(self) } }
}