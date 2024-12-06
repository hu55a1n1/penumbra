use penumbra_proto::penumbra::core::component::shielded_graph::v1 as pb;

use penumbra_proto::DomainType;
use serde::{Deserialize, Serialize};

use crate::fmd;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(
    try_from = "pb::ShieldedGraphParameters",
    into = "pb::ShieldedGraphParameters"
)]
pub struct ShieldedGraphParameters {
    pub fmd_meta_params: fmd::MetaParameters,
}

impl DomainType for ShieldedGraphParameters {
    type Proto = pb::ShieldedGraphParameters;
}

impl TryFrom<pb::ShieldedGraphParameters> for ShieldedGraphParameters {
    type Error = anyhow::Error;

    fn try_from(msg: pb::ShieldedGraphParameters) -> anyhow::Result<Self> {
        Ok(ShieldedGraphParameters {
            fmd_meta_params: msg
                .fmd_meta_params
                .ok_or_else(|| anyhow::anyhow!("missing fmd_meta_params"))?
                .try_into()?,
        })
    }
}

impl From<ShieldedGraphParameters> for pb::ShieldedGraphParameters {
    fn from(params: ShieldedGraphParameters) -> Self {
        #[allow(deprecated)]
        pb::ShieldedGraphParameters {
            fmd_meta_params: Some(params.fmd_meta_params.into()),
            fixed_fmd_params: None,
        }
    }
}
