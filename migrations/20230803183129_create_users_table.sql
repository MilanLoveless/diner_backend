-- Create Users Table
CREATE TABLE users(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   discord_id TEXT NOT NULL,
   avatar TEXT NOT NULL,
   created_at timestamptz NOT NULL,
   updated_at timestamptz NOT NULL
);
