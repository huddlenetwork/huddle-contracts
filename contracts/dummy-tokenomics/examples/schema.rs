use std::{env::current_dir, fs::create_dir_all};
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cw_desmos_dummy_tokenomics::{msg::SudoMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(SudoMsg), &out_dir);
}
