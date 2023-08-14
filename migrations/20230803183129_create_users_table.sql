-- Create Users Table
CREATE TABLE users(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   username TEXT NOT NULL,
   avatar TEXT,
   banner TEXT,
   global_name TEXT NOT NULL,
   created_at timestamptz NOT NULL,
   updated_at timestamptz NOT NULL
);
