use crate::tests::setup::{OWNER, SUBDENOM, USER};
use crate::{
    contract::{instantiate, query},
    msg::{InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
};
use cw20::TokenInfoResponse;

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
    let _response = result.unwrap();

    let token_info =
        from_json::<TokenInfoResponse>(&query(deps.as_ref(), env, QueryMsg::TokenInfo {}).unwrap())
            .unwrap();
    assert_eq!(token_info.symbol, SUBDENOM);
}
