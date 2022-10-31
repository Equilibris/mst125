#[derive(Default)]
pub struct WorkingsMonad<T> {
    workings: Vec<String>,
    inner: T,
}

impl<T> AsMut<T> for WorkingsMonad<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
impl<T> AsRef<T> for WorkingsMonad<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> WorkingsMonad<T> {
    pub fn new(v: T) -> Self {
        Self {
            inner: v,
            workings: Vec::new(),
        }
    }
    pub fn and_then<B, F: FnOnce(T) -> WorkingsMonad<B>>(self, f: F) -> WorkingsMonad<B> {
        let v = f.call_once((self.inner,));

        WorkingsMonad {
            workings: [self.workings, v.workings].concat(),
            inner: v.inner,
        }
    }
    pub fn map<B, F: FnOnce(T) -> B>(self, f: F) -> WorkingsMonad<B> {
        WorkingsMonad {
            workings: self.workings,
            inner: f.call_once((self.inner,)),
        }
    }
    pub fn add_workings(&mut self, s: String) {
        self.workings.push(s);
    }
    pub fn workings(mut self, s: String) -> Self {
        self.add_workings(s);
        self
    }
}
