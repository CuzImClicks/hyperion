use flecs_ecs::prelude::*;

pub struct SendableRef<'a>(pub WorldRef<'a>);

unsafe impl<'a> Send for SendableRef<'a> {}
unsafe impl<'a> Sync for SendableRef<'a> {}

pub struct SendableQuery<T>(pub Query<T>)
where
    T: QueryTuple;

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T: QueryTuple + Send> Send for SendableQuery<T> {}
unsafe impl<T: QueryTuple> Sync for SendableQuery<T> {}