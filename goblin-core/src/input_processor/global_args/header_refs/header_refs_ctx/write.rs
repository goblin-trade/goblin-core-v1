use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use crate::types::Address;

use super::HeaderRefsCtx;

impl<'a> HeaderRefsCtx<'a> {
    /// Write the optional custom recipient slot when the flags ask for it.
    pub fn write_recipient<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        recipient: &Option<&Address>,
    ) -> Result<(), DekuError> {
        if !self.flags.read_custom_recipient {
            return Ok(());
        }

        match recipient {
            // A well-formed value pairs the flag with `Some`; if it is missing,
            // still emit the slot as zeroes so the layout stays decodable.
            Some(address) => address.to_writer(writer, ()),
            None => [0u8; 20].to_writer(writer, ()),
        }
    }
}
