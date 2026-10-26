use systemconfiguration::SystemConfiguration;

#[test]
fn system_configuration_error_helpers_are_available() -> Result<(), Box<dyn std::error::Error>> {
    assert_ne!(SystemConfiguration::error_domain()?, "");
    let _ = SystemConfiguration::copy_last_error()?;
    Ok(())
}
