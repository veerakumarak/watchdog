-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS trg_global_settings_changed ON global_settings;
DROP FUNCTION IF EXISTS notify_settings_changed();
