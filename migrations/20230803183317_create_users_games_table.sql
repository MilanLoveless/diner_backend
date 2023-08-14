-- Create Users Games Table
CREATE TABLE users_games(
    user_id uuid REFERENCES users(id) ON DELETE CASCADE,
    game_id uuid REFERENCES games(id) ON DELETE CASCADE
);
