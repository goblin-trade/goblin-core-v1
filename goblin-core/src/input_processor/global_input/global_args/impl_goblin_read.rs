use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::input_processor::{GlobalArgs, Header, HeaderFlags, HeaderRefs, HeaderRefsCtx};

impl<'de> GoblinRead<'de, ()> for GlobalArgs<'de> {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let flags = HeaderFlags::from_reader_with_ctx(reader, ())?;
        let header = Header::from_reader_with_ctx(reader, flags)?;
        let refs = HeaderRefs::from_reader_with_ctx(reader, HeaderRefsCtx { flags })?;

        Ok(Self {
            flags,
            header,
            refs,
        })
    }
}
