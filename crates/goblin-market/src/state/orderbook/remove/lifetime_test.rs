pub struct InnerStruct<'a> {
    pub data: &'a mut u8,
}

pub trait InnerStructTrait<'a> {
    fn data(&mut self) -> &mut u8;
}

impl<'a> InnerStructTrait<'a> for InnerStruct<'a> {
    fn data(&mut self) -> &mut u8 {
        self.data
    }
}

pub struct OuterStruct<'a> {
    pub inner: InnerStruct<'a>,
}

pub trait OuterStructTrait<'a> {
    fn inner(&mut self) -> &mut InnerStruct<'a>;

    fn inner_v2(&'a mut self) -> &mut impl InnerStructTrait;

    fn use_inner(&'a mut self) {
        let inner = self.inner_v2();
        *inner.data() = 10;
    }
}

impl<'a> OuterStructTrait<'a> for OuterStruct<'a> {
    fn inner(&mut self) -> &mut InnerStruct<'a> {
        &mut self.inner
    }

    fn inner_v2(&'a mut self) -> &mut impl InnerStructTrait {
        &mut self.inner
    }
}
