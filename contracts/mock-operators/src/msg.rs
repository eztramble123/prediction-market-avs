use cosmwasm_schema::cw_serde;
// This pulls in the queries
pub use lavs_apis::interfaces::voting::*;

#[cw_serde]
pub struct InstantiateMsg {
    pub operators: Vec<InstantiateOperator>,
    /// The address of the Oracle Verifier contract
    pub oracle_verifier: String,
}

#[cw_serde]
pub struct InstantiateOperator {
    /// The address of the operator
    pub addr: String,
    /// Their voting power
    pub voting_power: u32,
}

impl InstantiateOperator {
    pub fn new(addr: String, voting_power: u32) -> Self {
        Self { addr, voting_power }
    }
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[cw_orch(disable_fields_sorting)]
pub enum ExecuteMsg {
    /// Allows an operator to submit a vote for a specific task
    SubmitVote {
        task_id: TaskId,
        result: Decimal,
    },
}