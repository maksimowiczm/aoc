pub trait NextPipe<T> {
    fn next(&self, from: &(T, T)) -> Option<(T, T)>;
}
