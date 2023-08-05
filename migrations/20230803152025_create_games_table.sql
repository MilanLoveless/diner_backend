-- Create Games Table
CREATE TABLE games(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   name TEXT NOT NULL,
   description TEXT NOT NULL,
   link TEXT NOT NULL,
   created_at timestamptz NOT NULL,
   updated_at timestamptz NOT NULL
);
