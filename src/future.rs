pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

struct Pin {
    
}

enum Poll<T> {
    Ready(T),
    Pending,
}