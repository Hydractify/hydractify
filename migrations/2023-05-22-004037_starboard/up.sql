CREATE TABLE IF NOT EXISTS starboard (
  message_id BIGINT PRIMARY KEY,
  starboard_id BIGINT NULL,
  stars INT NOT NULL
);
