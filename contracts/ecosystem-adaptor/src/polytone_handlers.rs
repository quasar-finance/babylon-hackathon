use cosmwasm_std::{
    ensure, Binary, DepsMut, Env, MessageInfo,
};

use polytone::callbacks::{
    Callback as PolytoneCallback, CallbackMessage, ErrorResponse, ExecutionResponse
};

use crate::{contract::AdaptorResult, state::POLYTONE_INFO, AdaptorError};


/// attempts to advance the state machine. performs `info.sender` validation.
pub fn try_handle_callback(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
    msg: CallbackMessage,
) -> AdaptorResult {
    // only the note can submit a callback
    ensure!(
        info.sender == POLYTONE_INFO.load(deps.storage)?.polyton_note_contract,
        AdaptorError::Unauthorized {}
    );

    match msg.result {
        PolytoneCallback::Query(resp) => process_query_callback(env, deps, resp, msg.initiator_msg),
        PolytoneCallback::Execute(resp) => {
            process_execute_callback(env, deps, resp, msg.initiator_msg)
        }
        PolytoneCallback::FatalError(resp) => process_fatal_error_callback(env, deps, resp),
    }
}

fn process_query_callback(
    _env: Env,
    _deps: DepsMut,
    _query_callback_result: Result<Vec<Binary>, ErrorResponse>,
    _initiator_msg: Binary,
) -> AdaptorResult {
    unimplemented!();
}


fn process_execute_callback(
    _env: Env,
    _deps: DepsMut,
    _execute_callback_result: Result<ExecutionResponse, String>,
    _initiator_msg: Binary,
) -> AdaptorResult {
    unimplemented!();
}

fn process_fatal_error_callback(
    _env: Env,
    _deps: DepsMut,
    _response: String,
) -> AdaptorResult {
    unimplemented!();
}