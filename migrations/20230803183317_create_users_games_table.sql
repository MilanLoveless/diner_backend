-- Create Users Games Table
CREATE TABLE users_games(
    user_id uuid REFERENCES users(id),
    game_id uuid REFERENCES games(id)
);
