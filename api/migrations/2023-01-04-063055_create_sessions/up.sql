-- Your SQL goes here
CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  ip_address CIDR NOT NULL,
  user_agent TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('sessions');