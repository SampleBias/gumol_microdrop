//! Build Nuclera-aligned expression screen matrices (construct × cell-free blend).

use crate::eprotein::models::{
    CellFreeBlend, Construct, ExpressionScreenMatrix, ExpressionScreenSlot, EproteinScreenKind,
    NUCLERA_N_BLENDS,
};

pub struct EproteinScreenDesigner;

impl EproteinScreenDesigner {
    /// Row-major: for each construct slot (0..24 or 0..11), for each blend (0..8).
    /// Uses `constructs[ci % len]` when non-empty; otherwise placeholder IDs.
    pub fn build_screen(
        kind: EproteinScreenKind,
        constructs: &[Construct],
        blends: &[CellFreeBlend],
    ) -> ExpressionScreenMatrix {
        let n_constructs = kind.n_construct_slots();
        let n_points = kind.n_expression_points();
        let mut slots = Vec::with_capacity(n_points);
        let mut slot_index = 0usize;

        for ci in 0..n_constructs {
            for bi in 0..NUCLERA_N_BLENDS {
                let blend = blends
                    .iter()
                    .find(|b| b.blend_index == bi)
                    .or_else(|| blends.get(bi));

                let (blend_name, blend_enabled) = match blend {
                    Some(b) => (b.name.clone(), b.enabled),
                    None => (format!("Blend {}", bi + 1), true),
                };

                let construct_id = if constructs.is_empty() {
                    format!("construct_{}", ci + 1)
                } else {
                    constructs[ci % constructs.len()].id.clone()
                };

                slots.push(ExpressionScreenSlot {
                    slot_index,
                    construct_index: ci,
                    blend_index: bi,
                    construct_id,
                    blend_name,
                    blend_enabled,
                });
                slot_index += 1;
            }
        }

        debug_assert_eq!(slots.len(), n_points);

        ExpressionScreenMatrix { kind, slots }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eprotein::models::NUCLERA_SOLUBLE_EXPRESSION_POINTS;

    #[test]
    fn soluble_full_grid_192() {
        let constructs: Vec<Construct> = (0..24)
            .map(|i| Construct::new(format!("g{}", i), format!("Gene {}", i)))
            .collect();
        let blends = CellFreeBlend::default_grid();
        let m = EproteinScreenDesigner::build_screen(
            EproteinScreenKind::SolubleProtein,
            &constructs,
            &blends,
        );
        assert_eq!(m.slots.len(), NUCLERA_SOLUBLE_EXPRESSION_POINTS);
        assert_eq!(m.kind.n_expression_points(), 192);
    }

    #[test]
    fn membrane_grid_88() {
        let constructs: Vec<Construct> = (0..11)
            .map(|i| Construct::new(format!("m{}", i), format!("Mem {}", i)))
            .collect();
        let blends = CellFreeBlend::default_grid();
        let m = EproteinScreenDesigner::build_screen(
            EproteinScreenKind::MembraneProtein,
            &constructs,
            &blends,
        );
        assert_eq!(m.slots.len(), 88);
    }
}
