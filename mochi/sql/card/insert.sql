INSERT INTO
  card ("name", description)
VALUES
  ($1, $2)
RETURNING
  id
