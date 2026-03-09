pub struct GenericIterator<K0, IT0, K1, IT1>
where
    IT0: Iterator<Item = K0>,
    IT1: Iterator<Item = K1>,
{
    pub outer_iterator: IT0,
    pub outer_item: K0,
    pub linear_iterator: IT1,
    pub limit: K1,
}
