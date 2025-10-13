import { Hono } from "hono";
import { zValidator } from "@hono/zod-validator";
import { CreateQueueSchema } from "@/shared/types";

const app = new Hono<{ Bindings: Env }>();

// Helper function to get Indonesia date
function getIndonesiaDate(): string {
  const now = new Date();
  const indonesiaTime = new Date(now.getTime() + 7 * 60 * 60 * 1000); // UTC+7
  return indonesiaTime.toISOString().split("T")[0];
}

// Helper function to clean old queues (older than 7 days)
async function cleanOldQueues(db: D1Database): Promise<void> {
  const sevenDaysAgo = new Date();
  sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7);
  const cutoffDate = sevenDaysAgo.toISOString().split("T")[0];

  await db
    .prepare(
      `
    DELETE FROM queues 
    WHERE DATE(created_at) < ?
  `
    )
    .bind(cutoffDate)
    .run();
}

// Get current queue numbers for both lokets
app.get("/api/queues/current", async (c) => {
  const db = c.env.DB;

  // Clean old queues before getting current counts
  await cleanOldQueues(db);

  const today = getIndonesiaDate();

  const loketA = await db
    .prepare(
      `
    SELECT COUNT(*) as count 
    FROM queues 
    WHERE loket_type = 'A' AND DATE(created_at) = ?
  `
    )
    .bind(today)
    .first();

  const loketB = await db
    .prepare(
      `
    SELECT COUNT(*) as count 
    FROM queues 
    WHERE loket_type = 'B' AND DATE(created_at) = ?
  `
    )
    .bind(today)
    .first();

  return c.json({
    loketA: loketA?.count || 0,
    loketB: loketB?.count || 0,
  });
});

// Create new queue number
app.post("/api/queues", zValidator("json", CreateQueueSchema), async (c) => {
  const { loket_type, patient_type } = c.req.valid("json");
  const db = c.env.DB;

  // Clean old queues before creating new one
  await cleanOldQueues(db);

  const today = getIndonesiaDate();

  // Get next queue number for the loket
  const result = await db
    .prepare(
      `
    SELECT COUNT(*) as count 
    FROM queues 
    WHERE loket_type = ? AND DATE(created_at) = ?
  `
    )
    .bind(loket_type, today)
    .first();

  const nextNumber = (Number(result?.count) || 0) + 1;
  const queueCode = `${loket_type}${String(nextNumber).padStart(3, "0")}`;

  // Insert new queue with Indonesia timezone
  const indonesiaTime = new Date();
  indonesiaTime.setTime(indonesiaTime.getTime() + 7 * 60 * 60 * 1000); // UTC+7

  await db
    .prepare(
      `
    INSERT INTO queues (loket_type, queue_number, queue_code, patient_type, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?, ?)
  `
    )
    .bind(
      loket_type,
      nextNumber,
      queueCode,
      patient_type,
      indonesiaTime.toISOString(),
      indonesiaTime.toISOString()
    )
    .run();

  return c.json({
    success: true,
    queue_code: queueCode,
    queue_number: nextNumber,
    loket_type,
    patient_type,
    timestamp: indonesiaTime.toISOString(),
  });
});

// Manual reset endpoint (optional - for admin use)
app.post("/api/queues/reset", async (c) => {
  const db = c.env.DB;

  // Delete all queues from today
  const today = getIndonesiaDate();
  await db
    .prepare(
      `
    DELETE FROM queues 
    WHERE DATE(created_at) = ?
  `
    )
    .bind(today)
    .run();

  return c.json({
    success: true,
    message: "Antrian hari ini telah direset",
  });
});

export default app;
