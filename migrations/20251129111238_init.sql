CREATE TABLE IF NOT EXISTS server (
    -- Uniquely identify a Minecraft server
    id TEXT UNIQUE NOT NULL,
    -- Arbitrary name to display to the user in the web dashboard
    name TEXT NOT NULL DEFAULT 'Unnamed Server',
    -- Path to the Minecraft server directory
    path TEXT UNIQUE NOT NULL,
    PRIMARY KEY (id)
)
