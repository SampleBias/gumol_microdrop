use serde::{Deserialize, Serialize};

/// Nuclera marketing numbers: soluble screen = 24 eGene × 8 blends = 192 expression points;
/// membrane = 11 × 8 = 88.
pub const NUCLERA_N_BLENDS: usize = 8;
pub const NUCLERA_SOLUBLE_N_CONSTRUCTS: usize = 24;
pub const NUCLERA_MEMBRANE_N_CONSTRUCTS: usize = 11;
pub const NUCLERA_SOLUBLE_EXPRESSION_POINTS: usize =
    NUCLERA_SOLUBLE_N_CONSTRUCTS * NUCLERA_N_BLENDS; // 192
pub const NUCLERA_MEMBRANE_EXPRESSION_POINTS: usize =
    NUCLERA_MEMBRANE_N_CONSTRUCTS * NUCLERA_N_BLENDS; // 88

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum DnaTopology {
    #[default]
    Linear,
    Circular,
}

/// One protein/DNA construct entering the design → screen pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Construct {
    pub id: String,
    pub display_name: String,
    /// Amino acid sequence (single-letter). Empty until user pastes/imports.
    pub amino_acid_sequence: String,
    pub topology: DnaTopology,
    /// Optional solubility tag description (e.g. MBP, TrxA).
    pub solubility_tag_note: String,
    /// Reserved for Phase 1 prediction API results (e.g. solubility score).
    pub prediction_solubility_score: Option<f64>,
    pub prediction_notes: String,
}

impl Construct {
    pub fn new(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            amino_acid_sequence: String::new(),
            topology: DnaTopology::default(),
            solubility_tag_note: String::new(),
            prediction_solubility_score: None,
            prediction_notes: String::new(),
        }
    }
}

/// One of eight customizable cell-free expression blends on Nuclera cartridges.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellFreeBlend {
    pub blend_index: usize, // 0..7
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

impl CellFreeBlend {
    pub fn default_grid() -> Vec<Self> {
        (0..NUCLERA_N_BLENDS)
            .map(|i| Self {
                blend_index: i,
                name: format!("Blend {}", i + 1),
                description: String::new(),
                enabled: true,
            })
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum EproteinScreenKind {
    /// 24 constructs × 8 blends = 192 expression data points.
    #[default]
    SolubleProtein,
    /// 11 constructs × 8 blends = 88 expression data points.
    MembraneProtein,
}

impl EproteinScreenKind {
    pub fn n_construct_slots(self) -> usize {
        match self {
            Self::SolubleProtein => NUCLERA_SOLUBLE_N_CONSTRUCTS,
            Self::MembraneProtein => NUCLERA_MEMBRANE_N_CONSTRUCTS,
        }
    }

    pub fn n_expression_points(self) -> usize {
        match self {
            Self::SolubleProtein => NUCLERA_SOLUBLE_EXPRESSION_POINTS,
            Self::MembraneProtein => NUCLERA_MEMBRANE_EXPRESSION_POINTS,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::SolubleProtein => "Soluble protein (24 × 8 = 192)",
            Self::MembraneProtein => "Membrane protein (11 × 8 = 88)",
        }
    }
}

/// One slot in the expression screen (construct × blend).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpressionScreenSlot {
    /// 0 .. n_expression_points-1 (row-major: construct major, blend minor).
    pub slot_index: usize,
    pub construct_index: usize,
    pub blend_index: usize,
    pub construct_id: String,
    pub blend_name: String,
    /// From `CellFreeBlend::enabled`; slot still exists for fixed 192/88 layout.
    pub blend_enabled: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExpressionScreenMatrix {
    pub kind: EproteinScreenKind,
    pub slots: Vec<ExpressionScreenSlot>,
}

/// All eProtein-specific state (separate from Gumol `ApplicationState` fields).
#[derive(Clone, Debug)]
pub struct EproteinProjectState {
    pub project_name: String,
    pub screen_kind: EproteinScreenKind,
    pub constructs: Vec<Construct>,
    pub blends: Vec<CellFreeBlend>,
    pub screen: Option<ExpressionScreenMatrix>,
}

impl Default for EproteinProjectState {
    fn default() -> Self {
        Self {
            project_name: "eProtein screen".to_string(),
            screen_kind: EproteinScreenKind::default(),
            constructs: Vec::new(),
            blends: CellFreeBlend::default_grid(),
            screen: None,
        }
    }
}
