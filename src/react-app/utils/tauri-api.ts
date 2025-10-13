import { invoke } from "@tauri-apps/api/core";

export interface QueueData {
  id: number;
  loket_type: string;
  queue_number: number;
  queue_code: string;
  patient_type: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface QueueCounts {
  loket_a: number;
  loket_b: number;
}

export interface CreateQueueRequest {
  loket_type: string;
  patient_type: string;
}

// Tauri API functions
export const tauriAPI = {
  // Get current queue counts
  async getCurrentQueues(): Promise<QueueCounts> {
    try {
      const result = await invoke<QueueCounts>("get_current_queues");
      return result;
    } catch (error) {
      console.error("Error getting current queues:", error);
      throw error;
    }
  },

  // Create new queue
  async createQueue(request: CreateQueueRequest): Promise<QueueData> {
    try {
      const result = await invoke<QueueData>("create_new_queue", { request });
      return result;
    } catch (error) {
      console.error("Error creating queue:", error);
      throw error;
    }
  },

  // Print ticket using browser's print function
  async printTicket(): Promise<void> {
    try {
      // Use browser's print function
      window.print();
    } catch (error) {
      console.error("Error printing ticket:", error);
      throw error;
    }
  },
};
