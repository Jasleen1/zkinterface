use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface::{
    Witness,
    WitnessArgs,
    Message,
    Root,
    RootArgs,
};
use super::variables::VariablesOwned;
use crate::Result;


#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct WitnessOwned {
    pub assigned_variables: VariablesOwned,
}

impl<'a> From<Witness<'a>> for WitnessOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(witness_ref: Witness) -> WitnessOwned {
        WitnessOwned {
            assigned_variables: VariablesOwned::from(witness_ref.assigned_variables().unwrap()),
        }
    }
}

impl WitnessOwned {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let assigned_variables = Some(self.assigned_variables.build(builder));

        let call = Witness::create(builder, &WitnessArgs {
            assigned_variables,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Witness,
            message: Some(call.as_union_value()),
        })
    }

    /// Writes this witness as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let witness = zkinterface::WitnessOwned::default();
    /// witness.write_into(&mut buf).unwrap();
    /// assert!(buf.len() > 0);
    /// ```
    pub fn write_into(&self, writer: &mut impl Write) -> Result<()> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        builder.finish_size_prefixed(message, None);
        writer.write_all(builder.finished_data())?;
        Ok(())
    }
}
