#[cfg(test)]
mod tests {
    use crate::{
        contract::{execute, instantiate, query},
        error::AdaptorError,
        msg::{BalanceResponse, EcosystemInfo, ExecuteMsg, InstantiateMsg, PolytoneInfo, QueryMsg},
        state::{BABYLON_VAULT, ECOSYSTEM_INFO, POLYTONE_INFO},
        tests::setup::{setup_with_balances, DENOM0, DENOM1, DENOM2, USER, VAULT},
    };

    use cosmwasm_std::{
        coin, from_json,
        testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, BankMsg, Coin, CosmosMsg, Uint128,
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            babylon_vault: VAULT.to_string(),
            ecosystem_info: EcosystemInfo {
                deposit_denoms: vec![DENOM0.to_string(), DENOM1.to_string(), DENOM2.to_string()],
                deposit_ecosystem: "cosmoshub".to_string(),
                transfer_channel: "channel-0".to_string(),
                connection: "connection-0".to_string(),
                return_source_channel: "channel-0".to_string(),
                destination_chain_denom: "ibc/denom".to_string(),
            },
            polytone_info: PolytoneInfo {
                polyton_note_contract: "polytone".to_string(),
            },
        };

        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Validate the state changes in the contract
        let vault = BABYLON_VAULT.load(&deps.storage).unwrap();
        assert_eq!(vault, Addr::unchecked(VAULT));

        let ecosystem_info = ECOSYSTEM_INFO.load(&deps.storage).unwrap();
        assert_eq!(
            ecosystem_info.deposit_denoms,
            vec![DENOM0.to_string(), DENOM1.to_string(), DENOM2.to_string()]
        );
        assert_eq!(ecosystem_info.deposit_ecosystem, "cosmoshub".to_string());

        let polytone_info = POLYTONE_INFO.load(&deps.storage).unwrap();
        assert_eq!(polytone_info.polyton_note_contract, "polytone".to_string());
    }

    #[test]
    fn test_balance_query() {
        // Setup with initial deposits
        let deps = setup_with_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[coin(1000, DENOM0), coin(2000, DENOM1), coin(4000, DENOM2)],
        )]);

        // Querying the balance
        let query_msg = QueryMsg::BalanceQuery {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();

        // Deserialize the response into BalanceResponse
        let balance_response: BalanceResponse = from_json(&res).unwrap();

        // Check the balance length and values
        assert_eq!(balance_response.balance.len(), 3);

        // Check each balance entry
        assert_eq!(
            balance_response.balance[0],
            Coin {
                denom: DENOM0.to_string(),
                amount: Uint128::new(1000)
            }
        );
        assert_eq!(
            balance_response.balance[1],
            Coin {
                denom: DENOM1.to_string(),
                amount: Uint128::new(2000)
            }
        );
        assert_eq!(
            balance_response.balance[2],
            Coin {
                denom: DENOM2.to_string(),
                amount: Uint128::new(4000)
            }
        );
    }

    #[test]
    fn test_deposit_unauthorized() {
        // Setup with initial balances
        let mut deps = setup_with_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[coin(1000, DENOM0), coin(2000, DENOM1)],
        )]);

        let info = mock_info(USER, &[coin(1000, DENOM0)]); // Unauthorized user trying to deposit

        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Deposit {});

        // Check that unauthorized deposit fails
        assert_eq!(res.unwrap_err(), AdaptorError::Unauthorized {}.into());
    }

    #[test]
    fn test_deposit_authorized() {
        // Setup with initial balances
        let mut deps = setup_with_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[coin(1000, DENOM0), coin(2000, DENOM1)],
        )]);

        let info = mock_info(VAULT, &[coin(1000, DENOM0)]); // Authorized user (the vault)

        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Deposit {});

        // Check that deposit is successful
        assert!(res.is_ok());
    }

    #[test]
    fn test_withdraw_unauthorized() {
        // Setup with initial balances
        let mut deps = setup_with_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[coin(1000, DENOM0), coin(2000, DENOM1)],
        )]);

        let info = mock_info(USER, &[coin(1000, DENOM0)]); // Unauthorized user trying to withdraw

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Withdraw {
                amounts: vec![coin(1000, DENOM0)],
            },
        );

        // Check that unauthorized withdrawal fails
        assert_eq!(res.unwrap_err(), AdaptorError::Unauthorized {}.into());
    }

    #[test]
    fn test_withdraw_authorized() {
        // Setup with initial balances
        let mut deps = setup_with_balances(&[(
            MOCK_CONTRACT_ADDR,
            &[coin(1000, DENOM0), coin(2000, DENOM1)],
        )]);

        let info = mock_info(VAULT, &[coin(1000, DENOM0)]); // Authorized user (the vault)

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Withdraw {
                amounts: vec![coin(1000, DENOM0)],
            },
        );

        // Check that withdrawal is successful and a BankMsg is sent
        let res = res.unwrap();
        assert_eq!(res.messages.len(), 1);

        if let CosmosMsg::Bank(BankMsg::Send { to_address, amount }) = &res.messages[0].msg {
            assert_eq!(to_address, VAULT);
            assert_eq!(amount, &[coin(1000, DENOM0)]);
        } else {
            panic!("Expected BankMsg::Send");
        }
    }
}
