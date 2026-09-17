use crate::{
    goblin_error::GoblinError,
    input_processor::{
        ArgsReader, CompoundDecode, FixedCodec, GlobalArgs, HeaderFlags, VariableDecode,
        global_args::{global_header::GlobalHeader, hostio_fields::HostioFields},
    },
};

impl<'a> CompoundDecode<'a> for GlobalArgs<'a> {
    fn try_compound_decode(reader: &'a ArgsReader) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_fixed_decode(reader)?;
        let global_header = GlobalHeader::try_variable_decode(reader, &flags)?;
        let hostio_fields = HostioFields::try_new(flags.read_msg_value)?;

        Ok(Self {
            flags,
            global_header,
            hostio_fields,
        })
    }
}
