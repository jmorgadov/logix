use logix_sim::{
    pcmd, pexp,
    primitives::pasm::{ProgramUpdateType, PASM},
};

use crate::app_ui::board::ComponentInfo;

use super::Library;

pub fn flip_flops_lib() -> Library {
    Library::new(
        [(
            "JK Master-Slave (Falling edge)".into(),
            ComponentInfo::custom(
                "JKMS",
                vec!["J".into(), "CLK".into(), "K".into()],
                vec!["Q".into(), "!Q".into()],
                PASM::new(
                    ProgramUpdateType::InputChanges,
                    vec![
                        // Estimate falling edge
                        pcmd!(
                            mov,
                            "falling_edge",
                            // Falling edge detection (!last_clk && clk)
                            pexp!(and, pexp!(not, pexp!(var, "_i_1")), pexp!(var, "last_clk"))
                        ),
                        // Update last clk with current clk value
                        pcmd!(mov, "last_clk", pexp!(var, "_i_1")),
                        // Check if falling edge
                        pcmd!(cmp, pexp!(var, "falling_edge"), pexp!(data, true)),
                        // If not rising edge, jump to END
                        pcmd!(jne, "END"),
                        // Else ...
                        // Create JK variable
                        pcmd!(
                            mov,
                            "JK",
                            pexp!(bit_vec, pexp!(var, "_i_0"), pexp!(var, "_i_2"))
                        ),
                        // Check JK value and jump to the corresponding case
                        pcmd!(cmp, pexp!(var, "JK"), pexp!(data, "00")),
                        pcmd!(je, "CASE_00"),
                        pcmd!(cmp, pexp!(var, "JK"), pexp!(data, "10")),
                        pcmd!(je, "CASE_10"),
                        pcmd!(cmp, pexp!(var, "JK"), pexp!(data, "01")),
                        pcmd!(je, "CASE_01"),
                        pcmd!(cmp, pexp!(var, "JK"), pexp!(data, "11")),
                        pcmd!(je, "CASE_11"),
                        //
                        pcmd!(label, "CASE_00"),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "CASE_01"),
                        pcmd!(mov, "_o_0", pexp!(data, false)),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "CASE_10"),
                        pcmd!(mov, "_o_0", pexp!(data, true)),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "CASE_11"),
                        pcmd!(mov, "_o_0", pexp!(not, pexp!(var, "_o_0"))),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "END"),
                        // Update !Q
                        pcmd!(mov, "_o_1", pexp!(not, pexp!(var, "_o_0"))),
                    ],
                )
                .with_default_vars([("last_clk", false)].into()),
            ),
        )]
        .into(),
        [],
    )
}
