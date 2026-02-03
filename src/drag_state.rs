#[derive(Clone)] pub struct DragState<T: 'static> {
    pub data: Option<T>,
}
