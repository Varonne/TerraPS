use serde::{Deserialize, Serialize};

use crate::{NullVec, PlayerDataDeltaStatic};

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnconfirmedIdList {
    order_id_list: NullVec,
    #[serde(flatten)]
    pdd: PlayerDataDeltaStatic,
}
