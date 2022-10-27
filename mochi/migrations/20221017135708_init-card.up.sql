-- Add up migration script here
CREATE TABLE IF NOT EXISTS card (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  "name" VARCHAR(200) NOT NULL,
  description TEXT NOT NULL,
  creation_timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS card_linked_generic_logic(
  id UUID NOT NULL,
  generic_logic_id UUID NOT NULL,
  PRIMARY KEY (id, generic_logic_id),
  FOREIGN KEY (id) REFERENCES card(id) ON
  UPDATE
    RESTRICT ON DELETE CASCADE
);
