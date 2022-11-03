DELETE FROM
  card_linked_generic_logic
WHERE
  id = $1 and generic_logic_id = $2
RETURNING
  id
