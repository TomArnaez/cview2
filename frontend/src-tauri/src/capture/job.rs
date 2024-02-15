use uuid::Uuid;

pub trait DynJob: Send + Sync {
    fn id(&self) -> Uuid;
}

pub trait StatefulJob {
    
}