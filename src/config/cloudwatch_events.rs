#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleRule {
    pub name: String,
    pub schedule_expression: String,
}
