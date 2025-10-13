import z from "zod";

export const QueueSchema = z.object({
  id: z.number(),
  loket_type: z.string(),
  queue_number: z.number(),
  queue_code: z.string(),
  patient_type: z.string(),
  status: z.string(),
  created_at: z.string(),
  updated_at: z.string(),
});

export type QueueType = z.infer<typeof QueueSchema>;

export const CreateQueueSchema = z.object({
  loket_type: z.enum(['A', 'B']),
  patient_type: z.string(),
});

export type CreateQueueType = z.infer<typeof CreateQueueSchema>;
