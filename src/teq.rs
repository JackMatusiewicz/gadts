pub struct Teq<A, B> {
    to: fn(A) -> B,
    from: fn(B) -> A,
}

impl<A> Teq<A, A> {
    fn id(v: A) -> A {
        v
    }

    pub fn refl() -> Teq<A, A> {
        Teq {
            to: Teq::<A, A>::id,
            from: Teq::<A, A>::id,
        }
    }
}

impl<A, B> Teq<A, B> {
    pub fn clone(&self) -> Teq<A, B> {
        Teq {
            to: self.to,
            from: self.from,
        }
    }

    pub fn from(t: &Teq<A, B>, v: B) -> A {
        (t.from)(v)
    }

    pub fn to(t: &Teq<A, B>, v: A) -> B {
        (t.to)(v)
    }
}
