
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

CREATE INDEX idx_queues_loket_type ON queues(loket_type);
CREATE INDEX idx_queues_status ON queues(status);
CREATE INDEX idx_queues_created_at ON queues(created_at);
