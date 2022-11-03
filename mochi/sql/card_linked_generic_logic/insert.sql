INSERT INTO
  card_linked_generic_logic (
    id,
    generic_logic_id
  )
VALUES
  ($1, $2)
RETURNING
  id
