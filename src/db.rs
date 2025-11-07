use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use tracing::info;
use aws_sdk_dynamodb::{
    Client as DynamoClient,
};

/// One-time setup to create DynamoDB tables if they don't exist.
/// Uses basic provisioned throughput for simple testing.
pub async fn setup_dynamodb_tables(
    client: &DynamoClient,
    config_table: &str,
    run_table: &str,
) -> Result<(), aws_sdk_dynamodb::Error> {
    let tables = client.list_tables().send().await?;
    let table_names = tables.table_names();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(1)
        .write_capacity_units(1)
        .build()?;

    if !table_names.contains(&config_table.to_string()) {
        info!("Creating table: {}", config_table);
        client.create_table()
            .table_name(config_table)
            .key_schema(KeySchemaElement::builder()
                .attribute_name("job_name")
                .key_type(KeyType::Hash)
                .build()?
            )
            .attribute_definitions(AttributeDefinition::builder()
                .attribute_name("job_name")
                .attribute_type(ScalarAttributeType::S)
                .build()?
            )
            .provisioned_throughput(pt.clone())
            .send()
            .await?;
    }

    if !table_names.contains(&run_table.to_string()) {
        info!("Creating table: {}", run_table);
        client.create_table()
            .table_name(run_table)
            .key_schema(KeySchemaElement::builder()
                .attribute_name("run_id")
                .key_type(KeyType::Hash)
                .build()?
            )
            .attribute_definitions(AttributeDefinition::builder()
                .attribute_name("run_id")
                .attribute_type(ScalarAttributeType::S)
                .build()?
            )
            .provisioned_throughput(pt)
            .send()
            .await?;
    }

    info!("DynamoDB tables are ready.");
    Ok(())
}
