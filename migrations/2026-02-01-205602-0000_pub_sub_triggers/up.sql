-- Create a function that sends a JSON payload
CREATE OR REPLACE FUNCTION notify_settings_changed()
RETURNS trigger AS $$
DECLARE
payload JSON;
BEGIN
  -- Construct a JSON object containing the ID and the new value
  -- You can customize this to include any columns you need
  payload = json_build_object(
    'id', NEW.id,
    'success_retention_days', NEW.success_retention_days,
    'failure_retention_days', NEW.failure_retention_days,
    'maintenance_mode', NEW.maintenance_mode,
    'default_channels', NEW.default_channels,
    'max_stage_duration_hours', NEW.max_stage_duration_hours,
    'action', TG_OP
  );

  -- Perform the NOTIFY with the JSON string
  PERFORM pg_notify('settings_update', payload::text);

RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach the trigger to your table
CREATE TRIGGER trg_global_settings_changed
    AFTER UPDATE OR INSERT ON global_settings
    FOR EACH ROW -- Use EACH ROW to get specific row data in 'NEW'
EXECUTE FUNCTION notify_settings_changed();
