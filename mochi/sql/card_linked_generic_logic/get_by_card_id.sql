SELECT
  id,
  generic_logic_id
FROM
  card_linked_generic_logic
WHERE
  id = $1
