use crate::tests::setup::{OWNER, SUBDENOM, USER, VAULT_DENOM};
use crate::{
    contract::{instantiate, query, reply},
    msg::{InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    Reply, SubMsgResponse, SubMsgResult,
};
use cw20::TokenInfoResponse;
use prost::Message;
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse,
};

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let result = instantiate(
        deps.as_mut(),
        env.clone(),
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
            subdenom: SUBDENOM.to_string(),
            oracle: "oracle".to_string(),
            gauge: "gauge".to_string(),
        },
    );
    assert!(result.is_ok());
    let response = result.unwrap();

    let token_info =
        from_json::<TokenInfoResponse>(&query(deps.as_ref(), env, QueryMsg::TokenInfo {}).unwrap())
            .unwrap();
    assert_eq!(token_info.symbol, SUBDENOM);
}
