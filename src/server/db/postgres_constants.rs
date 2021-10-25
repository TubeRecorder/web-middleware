pub static INSERT_DOWNLOAD: &str = "
INSERT INTO
  downloads 
  (
    entry_id,
    link_url,
    local_path,
    status
  )
VALUES
  (
    $1,
    $2,
    $3,
    $4
  );
";

pub static UPDATE_DOWNLOAD: &str = "
UPDATE
  downloads
SET
  status = $1
WHERE
  entry_id = $2;
";

pub static DELETE_DOWNLOAD: &str = "
DELETE FROM
  downloads
WHERE
  entry_id = $1;
";

pub static SELECT_DOWNLOADS: &str = "
SELECT
  entry_id,
  link_url,
  local_path,
  status
FROM
  downloads
";

pub static INSERT_CONFIG: &str = "
INSERT INTO
  configs 
  (
    config_value,
    config_key
  )
VALUES
  (
    $1,
    $2
  );
";

pub static UPDATE_CONFIG: &str = "
UPDATE
  configs
SET
  config_value = $1
WHERE
  config_key = $2;
";

pub static SELECT_CONFIG: &str = "
SELECT
  config_value
FROM
  configs
WHERE
  config_key = $1;
";
