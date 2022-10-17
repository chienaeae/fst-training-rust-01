DELETE FROM
  card
WHERE
  id = $1
RETURNING
  id
