use anyhow::Result;
use proof_forge_core::groth16::VerifyingKey;

use std::fmt::Write;
pub fn build_vkey(vkey: &VerifyingKey) -> Result<String> {
    let mut s = String::new();

    writeln!(s, "    uint256 constant alphax = {};", vkey.alpha.x)?;
    writeln!(s, "    uint256 constant alphay = {};", vkey.alpha.y)?;

    writeln!(s, "    uint256 constant betax1 = {};", vkey.beta.x0)?;
    writeln!(s, "    uint256 constant betax2 = {};", vkey.beta.x1)?;
    writeln!(s, "    uint256 constant betay1 = {};", vkey.beta.y0)?;
    writeln!(s, "    uint256 constant betay2 = {};", vkey.beta.y1)?;

    writeln!(s, "    uint256 constant gammax1 = {};", vkey.gamma.x0)?;
    writeln!(s, "    uint256 constant gammax2 = {};", vkey.gamma.x1)?;
    writeln!(s, "    uint256 constant gammay1 = {};", vkey.gamma.y0)?;
    writeln!(s, "    uint256 constant gammay2 = {};", vkey.gamma.y1)?;

    writeln!(s, "    uint256 constant deltax1 = {};", vkey.delta.x0)?;
    writeln!(s, "    uint256 constant deltax2 = {};", vkey.delta.x1)?;
    writeln!(s, "    uint256 constant deltay1 = {};", vkey.delta.y0)?;
    writeln!(s, "    uint256 constant deltay2 = {};", vkey.delta.y1)?;

    writeln!(s)?;

    let ic = &vkey.ic;

    for i in 0..ic.len() {
        writeln!(s, "    uint256 constant IC{}x = {};", i, ic[i].x)?;
        writeln!(s, "    uint256 constant IC{}y = {};", i, ic[i].y)?;

        writeln!(s)?;
    }

    Ok(s)
}
