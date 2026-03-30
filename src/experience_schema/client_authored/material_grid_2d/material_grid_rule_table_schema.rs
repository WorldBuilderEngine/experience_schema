use crate::properties::property::Property;
use prost::{Enumeration, Message};
use serde::{Deserialize, Serialize};

pub const CURRENT_MATERIAL_GRID_2D_RULE_TABLES_FORMAT_VERSION: u32 = 1;
pub const CURRENT_MATERIAL_GRID_2D_RULE_TABLE_VERSION: u32 = 1;

fn default_material_grid_2d_rule_tables_format_version() -> u32 {
    CURRENT_MATERIAL_GRID_2D_RULE_TABLES_FORMAT_VERSION
}

fn default_material_grid_2d_rule_table_version() -> u32 {
    CURRENT_MATERIAL_GRID_2D_RULE_TABLE_VERSION
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enumeration)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum MaterialGrid2dRuleScanOrderSchema {
    RowMajorTopLeft = 0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enumeration)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum MaterialGrid2dRuleComparisonSchema {
    Equals = 0,
    NotEquals = 1,
    LessThan = 2,
    LessThanOrEqual = 3,
    GreaterThan = 4,
    GreaterThanOrEqual = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enumeration)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum MaterialGrid2dRuleWriteOperationSchema {
    SetConstant = 0,
    CopyField = 1,
    AddIntDelta = 2,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct MaterialGrid2dRuleConditionSchema {
    #[prost(string, tag = "1")]
    pub field_identifier: String,
    #[prost(int32, tag = "2")]
    pub offset_x: i32,
    #[prost(int32, tag = "3")]
    pub offset_y: i32,
    #[serde(default)]
    #[prost(enumeration = "MaterialGrid2dRuleComparisonSchema", tag = "4")]
    pub comparison: i32,
    #[serde(default)]
    #[prost(message, optional, tag = "5")]
    pub value: Option<Property>,
}

impl MaterialGrid2dRuleConditionSchema {
    pub fn new(
        field_identifier: impl Into<String>,
        comparison: MaterialGrid2dRuleComparisonSchema,
        value: Property,
    ) -> Self {
        Self {
            field_identifier: field_identifier.into(),
            offset_x: 0,
            offset_y: 0,
            comparison: comparison as i32,
            value: Some(value),
        }
    }

    pub fn with_offset(mut self, offset_x: i32, offset_y: i32) -> Self {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        self
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct MaterialGrid2dRuleWriteSchema {
    #[prost(string, tag = "1")]
    pub target_field_identifier: String,
    #[serde(default)]
    #[prost(enumeration = "MaterialGrid2dRuleWriteOperationSchema", tag = "2")]
    pub operation: i32,
    #[serde(default)]
    #[prost(message, optional, tag = "3")]
    pub value: Option<Property>,
    #[prost(string, tag = "4")]
    pub source_field_identifier: String,
    #[prost(int32, tag = "5")]
    pub source_offset_x: i32,
    #[prost(int32, tag = "6")]
    pub source_offset_y: i32,
    #[serde(default)]
    #[prost(int64, tag = "7")]
    pub int_delta: i64,
}

impl MaterialGrid2dRuleWriteSchema {
    pub fn set_constant(target_field_identifier: impl Into<String>, value: Property) -> Self {
        Self {
            target_field_identifier: target_field_identifier.into(),
            operation: MaterialGrid2dRuleWriteOperationSchema::SetConstant as i32,
            value: Some(value),
            source_field_identifier: String::new(),
            source_offset_x: 0,
            source_offset_y: 0,
            int_delta: 0,
        }
    }

    pub fn copy_field(
        target_field_identifier: impl Into<String>,
        source_field_identifier: impl Into<String>,
        source_offset_x: i32,
        source_offset_y: i32,
    ) -> Self {
        Self {
            target_field_identifier: target_field_identifier.into(),
            operation: MaterialGrid2dRuleWriteOperationSchema::CopyField as i32,
            value: None,
            source_field_identifier: source_field_identifier.into(),
            source_offset_x,
            source_offset_y,
            int_delta: 0,
        }
    }

    pub fn add_int_delta(
        target_field_identifier: impl Into<String>,
        source_field_identifier: impl Into<String>,
        source_offset_x: i32,
        source_offset_y: i32,
        int_delta: i64,
    ) -> Self {
        Self {
            target_field_identifier: target_field_identifier.into(),
            operation: MaterialGrid2dRuleWriteOperationSchema::AddIntDelta as i32,
            value: None,
            source_field_identifier: source_field_identifier.into(),
            source_offset_x,
            source_offset_y,
            int_delta,
        }
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct MaterialGrid2dRuleSchema {
    #[prost(string, tag = "1")]
    pub rule_id: String,
    #[serde(default)]
    #[prost(message, repeated, tag = "2")]
    pub conditions: Vec<MaterialGrid2dRuleConditionSchema>,
    #[serde(default)]
    #[prost(message, repeated, tag = "3")]
    pub writes: Vec<MaterialGrid2dRuleWriteSchema>,
}

impl MaterialGrid2dRuleSchema {
    pub fn new(rule_id: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            conditions: Vec::new(),
            writes: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: MaterialGrid2dRuleConditionSchema) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn with_write(mut self, write: MaterialGrid2dRuleWriteSchema) -> Self {
        self.writes.push(write);
        self
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct MaterialGrid2dRuleTableSchema {
    #[prost(string, tag = "1")]
    pub rule_table_id: String,
    #[prost(uint64, tag = "2")]
    pub compiled_rule_table_id: u64,
    #[serde(default = "default_material_grid_2d_rule_table_version")]
    #[prost(uint32, tag = "3")]
    pub rule_table_version: u32,
    #[prost(string, tag = "4")]
    pub layout_id: String,
    #[serde(default)]
    #[prost(enumeration = "MaterialGrid2dRuleScanOrderSchema", tag = "5")]
    pub scan_order: i32,
    #[serde(default)]
    #[prost(message, repeated, tag = "6")]
    pub rules: Vec<MaterialGrid2dRuleSchema>,
}

impl MaterialGrid2dRuleTableSchema {
    pub fn new(rule_table_id: impl Into<String>, layout_id: impl Into<String>) -> Self {
        let rule_table_id = rule_table_id.into();
        Self {
            compiled_rule_table_id: canonical_compiled_identifier(rule_table_id.as_str()),
            rule_table_id,
            rule_table_version: CURRENT_MATERIAL_GRID_2D_RULE_TABLE_VERSION,
            layout_id: layout_id.into(),
            scan_order: MaterialGrid2dRuleScanOrderSchema::RowMajorTopLeft as i32,
            rules: Vec::new(),
        }
    }

    pub fn with_rule(mut self, rule: MaterialGrid2dRuleSchema) -> Self {
        self.rules.push(rule);
        self
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct MaterialGrid2dRuleTablesSchema {
    #[serde(default = "default_material_grid_2d_rule_tables_format_version")]
    #[prost(uint32, tag = "1")]
    pub format_version: u32,
    #[serde(default)]
    #[prost(message, repeated, tag = "2")]
    pub rule_tables: Vec<MaterialGrid2dRuleTableSchema>,
}

impl MaterialGrid2dRuleTablesSchema {
    pub fn current() -> Self {
        Self {
            format_version: CURRENT_MATERIAL_GRID_2D_RULE_TABLES_FORMAT_VERSION,
            rule_tables: Vec::new(),
        }
    }
}

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

fn canonical_compiled_identifier(identifier: &str) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in identifier.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::{
        CURRENT_MATERIAL_GRID_2D_RULE_TABLES_FORMAT_VERSION, MaterialGrid2dRuleComparisonSchema,
        MaterialGrid2dRuleConditionSchema, MaterialGrid2dRuleSchema, MaterialGrid2dRuleTableSchema,
        MaterialGrid2dRuleTablesSchema, MaterialGrid2dRuleWriteOperationSchema,
        MaterialGrid2dRuleWriteSchema, canonical_compiled_identifier,
    };
    use crate::properties::property::Property;

    #[test]
    fn rule_table_schema_assigns_canonical_compiled_id() {
        let rule_table =
            MaterialGrid2dRuleTableSchema::new("falling_sand", "material_grid_runtime");

        assert_eq!(
            rule_table.compiled_rule_table_id,
            canonical_compiled_identifier("falling_sand")
        );
        assert_eq!(rule_table.layout_id, "material_grid_runtime");
    }

    #[test]
    fn rule_builders_preserve_conditions_and_writes() {
        let rule = MaterialGrid2dRuleSchema::new("fall")
            .with_condition(
                MaterialGrid2dRuleConditionSchema::new(
                    "material_id",
                    MaterialGrid2dRuleComparisonSchema::Equals,
                    Property::Int64(1),
                )
                .with_offset(0, -1),
            )
            .with_write(MaterialGrid2dRuleWriteSchema::add_int_delta(
                "timer_ticks",
                "timer_ticks",
                0,
                -1,
                1,
            ));

        assert_eq!(rule.conditions.len(), 1);
        assert_eq!(rule.conditions[0].offset_y, -1);
        assert_eq!(
            rule.writes[0].operation,
            MaterialGrid2dRuleWriteOperationSchema::AddIntDelta as i32
        );
        assert_eq!(rule.writes[0].int_delta, 1);
    }

    #[test]
    fn rule_tables_schema_defaults_to_current_format_version() {
        let rule_tables = MaterialGrid2dRuleTablesSchema::current();

        assert_eq!(
            rule_tables.format_version,
            CURRENT_MATERIAL_GRID_2D_RULE_TABLES_FORMAT_VERSION
        );
        assert!(rule_tables.rule_tables.is_empty());
    }
}
