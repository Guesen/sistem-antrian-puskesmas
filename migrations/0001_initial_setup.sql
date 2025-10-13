-- Migration 0001: Initial setup
-- Up
CREATE TABLE queues (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  loket_type TEXT NOT NULL,
  queue_number INTEGER NOT NULL,
  queue_code TEXT NOT NULL,
  patient_type TEXT NOT NULL,
  status TEXT DEFAULT 'waiting',
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Down
-- DROP TABLE queues;
