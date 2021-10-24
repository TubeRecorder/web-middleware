pub static INSERT_DOWNLOAD: &str = "
INSERT INTO
  downloads 
  (
    link_url,
    local_path,
    status
  )
VALUES
  (
    $1,
    $2,
    $3
  );
";

pub static UPDATE_DOWNLOAD: &str = "
UPDATE
  downloads
SET
  status = $1
WHERE
  link_url = $2
  AND
  local_path = $3;
";

pub static SELECT_DOWNLOADS: &str = "
SELECT
  link_url,
  local_path,
  status
FROM
  downloads
";
