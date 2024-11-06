#[cfg(test)]
mod tests {
    use crate::{
        contract::{execute, instantiate},
        error::AdaptorError,
        msg::{EcosystemInfo, ExecuteMsg, InstantiateMsg, PolytoneInfo},
        state::{BABYLON_VAULT, ECOSYSTEM_INFO, POLYTONE_INFO},
    };
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, BankMsg, Coin, Uint128};

    fn instantiate_message() -> InstantiateMsg {
        InstantiateMsg {
            babylon_vault: "babylon_vault_addr".to_string(),
            ecosystem_info: EcosystemInfo {
                deposit_denoms: vec![
                    "uatom".to_string(),
                    "uosmo".to_string(),
                    "ujuno".to_string(),
                ],
                deposit_ecosystem: "cosmoshub".to_string(),
                transfer_channel: "channel-0".to_string(),
                connection: "connection-0".to_string(),
                return_source_channel: "channel-0".to_string(),
                destination_chain_denom: "ibc/denom".to_string(),
            },
            polytone_info: PolytoneInfo {
                polyton_note_contract: "polytone".to_string(),
            },
        }
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let msg = instantiate_message();
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Validate the event attributes
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "instantiate");

        assert_eq!(res.attributes[1].key, "module");
        assert_eq!(res.attributes[1].value, "ecosystem_adaptor");

        assert_eq!(res.attributes[2].key, "babylon_vault");
        assert_eq!(res.attributes[2].value, "babylon_vault_addr");

        assert_eq!(res.attributes[3].key, "ecosystem");
        assert_eq!(res.attributes[3].value, "cosmoshub"); // Assuming "cosmoshub" as example

        assert_eq!(res.attributes[4].key, "deposit_denoms");
        assert_eq!(res.attributes[4].value, "[\"uatom\", \"uosmo\", \"ujuno\"]");

        assert_eq!(res.attributes[5].key, "polyton_note_contract");
        assert_eq!(res.attributes[5].value, "polytone");

        // Validate stored values in state
        let vault = BABYLON_VAULT.load(&deps.storage).unwrap();
        assert_eq!(vault, Addr::unchecked("babylon_vault_addr"));

        let ecosystem_info = ECOSYSTEM_INFO.load(&deps.storage).unwrap();
        assert_eq!(
            ecosystem_info.deposit_denoms,
            vec!["uatom".to_string(), "uosmo".to_string(), "ujuno".to_string()]
        );

        let polytone_info = POLYTONE_INFO.load(&deps.storage).unwrap();
        assert_eq!(polytone_info.polyton_note_contract, "polytone");
    }

    #[test]
    fn test_deposit_success_with_multiple_denoms() {
        let mut deps = mock_dependencies();

        // Instantiate the contract with multiple allowed denoms
        let msg = instantiate_message();

        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();

        // Test deposit with one of the allowed denoms ("uatom")
        let info = mock_info(
            "babylon_vault_addr",
            &[Coin {
                denom: "uatom".to_string(),
                amount: Uint128::new(500),
            }],
        );
        let msg = ExecuteMsg::Deposit {};
        let res = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();

        // Verify the deposit was successful
        assert_eq!(res.attributes[0].key, "deposit");
        assert_eq!(res.attributes[0].value, "500uatom");

        // Test deposit with another allowed denom ("uosmo")
        let info = mock_info(
            "babylon_vault_addr",
            &[
                Coin {
                    denom: "uosmo".to_string(),
                    amount: Uint128::new(1000),
                },
                Coin {
                    denom: "uatom".to_string(),
                    amount: Uint128::new(500),
                },
            ],
        );
        let res = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();

        // Verify the deposit was successful with "uosmo" and "uatoms"
        assert_eq!(res.attributes[0].key, "deposit");
        assert_eq!(res.attributes[0].value, "1000uosmo, 500uatom");

        // Test deposit with a non-allowed denom ("unallowed")
        let info = mock_info(
            "babylon_vault_addr",
            &[Coin {
                denom: "unallowed".to_string(),
                amount: Uint128::new(100),
            }],
        );
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        // Verify that the deposit failed with a DenomNotFound error
        assert_eq!(
            err,
            AdaptorError::DenomNotFound {
                denom: "unallowed".to_string()
            }
        );
    }

    #[test]
    fn test_deposit_invalid_denom() {
        let mut deps = mock_dependencies();
        let msg = instantiate_message();
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info(
            "babylon_vault_addr",
            &[Coin {
                denom: "invalid_denom".to_string(),
                amount: Uint128::new(500),
            }],
        );
        let msg = ExecuteMsg::Deposit {};
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert_eq!(
            err,
            AdaptorError::DenomNotFound {
                denom: "invalid_denom".to_string()
            }
        );
    }

    #[test]
    fn test_deposit_unauthorized() {
        let mut deps = mock_dependencies();
        let msg = instantiate_message();
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info(
            "random_user",
            &[Coin {
                denom: "uatom".to_string(),
                amount: Uint128::new(500),
            }],
        );
        let msg = ExecuteMsg::Deposit {};
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert_eq!(err, AdaptorError::Unauthorized {});
    }

    #[test]
    fn test_withdraw_success() {
        let mut deps = mock_dependencies();
        let msg = instantiate_message();
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info(
            "babylon_vault_addr",
            &[Coin {
                denom: "uatom".to_string(),
                amount: Uint128::new(1000),
            }],
        );
        let msg = ExecuteMsg::Withdraw {
            amounts: vec![Coin {
                denom: "uatom".to_string(),
                amount: Uint128::new(500),
            }],
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.attributes[0].key, "withdraw");
        assert_eq!(res.attributes[0].value, "500uatom");

        match &res.messages[0].msg {
            cosmwasm_std::CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                assert_eq!(to_address, "babylon_vault_addr");
                assert_eq!(amount[0].amount, Uint128::new(500));
                assert_eq!(amount[0].denom, "uatom");
            }
            _ => panic!("Unexpected message type"),
        }
    }

    #[test]
    fn test_withdraw_success_multiple_denoms() {
        let mut deps = mock_dependencies();

        // Instantiate the contract with multiple allowed denoms
        let msg = instantiate_message();

        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Deposit initial amounts for multiple denoms
        let info = mock_info(
            "babylon_vault_addr",
            &[
                Coin {
                    denom: "uatom".to_string(),
                    amount: Uint128::new(1000),
                },
                Coin {
                    denom: "uosmo".to_string(),
                    amount: Uint128::new(1500),
                },
            ],
        );
        let deposit_msg = ExecuteMsg::Deposit {};
        execute(deps.as_mut(), mock_env(), info.clone(), deposit_msg).unwrap();

        // Prepare a Withdraw message for multiple denoms
        let withdraw_msg = ExecuteMsg::Withdraw {
            amounts: vec![
                Coin {
                    denom: "uatom".to_string(),
                    amount: Uint128::new(500),
                },
                Coin {
                    denom: "uosmo".to_string(),
                    amount: Uint128::new(1000),
                },
            ],
        };

        // Execute the withdraw
        let res = execute(deps.as_mut(), mock_env(), info, withdraw_msg).unwrap();

        // Check the attributes in the response
        assert_eq!(res.attributes[0].key, "withdraw");
        assert_eq!(res.attributes[0].value, "500uatom, 1000uosmo");

        // Verify the BankMsg::Send for each denomination
        if let cosmwasm_std::CosmosMsg::Bank(BankMsg::Send { to_address, amount }) =
            &res.messages[0].msg
        {
            assert_eq!(to_address, "babylon_vault_addr");

            // Check that both denoms are included in the withdrawal message
            assert_eq!(amount.len(), 2);
            assert_eq!(amount[0].denom, "uatom");
            assert_eq!(amount[0].amount, Uint128::new(500));
            assert_eq!(amount[1].denom, "uosmo");
            assert_eq!(amount[1].amount, Uint128::new(1000));
        } else {
            panic!("Unexpected message type");
        }
    }

    #[test]
    fn test_withdraw_unauthorized() {
        let mut deps = mock_dependencies();
        let msg = instantiate_message();
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("random_user", &[]);
        let msg = ExecuteMsg::Withdraw {
            amounts: vec![Coin {
                denom: "uatom".to_string(),
                amount: Uint128::new(500),
            }],
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert_eq!(err, AdaptorError::Unauthorized {});
    }
}
