SELECT
  id,
  "name",
  description,
  creation_timestamp
FROM
  card
WHERE
  id = $1
LIMIT
  1
