use crate::{
    goblin_error::GoblinError,
    input_processor::{
        ArgsReader, CompoundDecode, FixedCodec, GlobalArgs, HeaderFlags, VariableDecode,
        global_args::{header::Header, header_refs::HeaderRefs, hostio_fields::HostioFields},
    },
};

impl<'a> CompoundDecode<'a> for GlobalArgs<'a> {
    fn try_compound_decode(reader: &'a ArgsReader) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_fixed_decode(reader)?;
        let header = Header::try_variable_decode(reader, &flags)?;
        let refs = HeaderRefs::try_variable_decode(reader, &flags)?;
        let hostio_fields = HostioFields::try_new(flags.read_msg_value)?;

        Ok(Self {
            flags,
            header,
            refs,
            hostio_fields,
        })
    }
}
