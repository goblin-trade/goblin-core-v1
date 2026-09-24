use goblin_types::{CodecResult, GoblinWrite, Writer};

use crate::input_processor::{GlobalArgs, HeaderRefsCtx};

impl<'de> GoblinWrite<()> for GlobalArgs<'de> {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.flags.to_writer(writer, ())?;
        self.header.to_writer(writer, self.flags)?;
        self.refs
            .to_writer(writer, HeaderRefsCtx { flags: self.flags })?;
        Ok(())
    }
}
