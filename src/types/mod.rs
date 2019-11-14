pub struct BoltValue {
    ty: BoltValueTy,
}

pub enum BoltValueTy {
    // Int types
    Integer(BoltIntTy),

    // Double
    Float,

    // Variably sized
    String(BoltStringTy),

    List(BoltListTy),

    Map(BoltMapTy),
}

pub enum BoltIntTy {
    Int64,
    Int32,
    Int16,
    Int8,
    TinyInt,
}

pub struct BoltStringTy {
    size: usize,
}

pub struct BoltListTy {
    size: usize,
}

pub struct BoltMapTy {
    size: usize,
}
