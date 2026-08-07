use crate::{
    goblin_error::GoblinError,
    input_processor::{
        global_args::{global_header::GlobalHeader, hostio_fields::HostioFields},
        CompoundDecode, DecodeCtx, FixedDecode, GlobalArgs, HeaderFlags, VariableDecode,
    },
};

impl<'a> CompoundDecode<'a> for GlobalArgs<'a> {
    fn try_compound_decode(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_fixed_decode(ctx)?;
        let global_header = GlobalHeader::try_variable_decode(ctx, &flags)?;
        let hostio_fields = HostioFields::try_new(flags.read_msg_value)?;

        Ok(Self {
            flags,
            global_header,
            hostio_fields,
        })
    }
}
