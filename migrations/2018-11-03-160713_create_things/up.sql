CREATE TABLE things (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Apply the trigger that will automatically maintain updated_at
SELECT diesel_manage_updated_at('things');

-- Put some seed data in there (for testing)
INSERT INTO things (name) VALUES ('foo');
INSERT INTO things (name) VALUES ('bar');


