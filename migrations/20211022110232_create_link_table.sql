CREATE TABLE links(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  path TEXT NOT NULL UNIQUE,
  destination TEXT NOT NULL,
  created_at timestamptz NOT NULL,
  modified_at timestamptz NOT NULL
);
