use std::array;
#[derive(Clone)]
pub struct Data<const N: usize> {
    bytes: [u8; N],
    next: Option<Box<Data<N>>>,
}
impl<const N: usize> Default for Data<N> {
    fn default() -> Self {
        let bytes = [0u8; N];
        Data { bytes, next: None }
    }
}

impl<const N: usize> Data<N> {
    pub fn next(&mut self, data: Data<N>) {
        self.next = Some(Box::new(data));
    }

    pub fn create_list(size: usize) -> Self {
        let mut result = Data::default();
        let mut temp_result = &mut result;
        for _ in 0..size {
            temp_result.next(Data::default());
            temp_result = result.next.as_deref_mut().unwrap()
        }
        result
    }
}

pub fn array_access<const N: usize>(array: [[u8; N]; 1000]) -> usize {
    let mut c: usize = 0;
    let v = array::IntoIter::new(array);
    for i in v {
        c += i.len();
    }
    c
}

pub fn data_access<const N: usize>(data: &Data<N>) -> usize {
    let mut c: usize = 0;
    let mut d = data;
    while let Some(next) = d.next.as_deref() {
        d = next;
        c += d.bytes.len();
    }
    c
}
