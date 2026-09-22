use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use super::CustomERC20List;

/// Encode the list as the concatenated token addresses, mirroring the
/// zero-copy slice read. Each `TokenData<CustomERC20>` is just its `Address`
/// (20 bytes); the decimals marker is zero-sized.
impl<'a> DekuWriter<()> for CustomERC20List<'a> {
    fn to_writer<W: Write + Seek>(&self, writer: &mut Writer<W>, _: ()) -> Result<(), DekuError> {
        for token in self.inner {
            token.address.to_writer(writer, ())?;
        }
        Ok(())
    }
}
