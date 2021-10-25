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
