import { useState } from "react";
import { Printer } from "lucide-react";
import { tauriAPI, type QueueData } from "../utils/tauri-api";

interface QueueButtonProps {
  loket: "A" | "B";
  title: string;
  subtitle: string;
  onQueueGenerated: (queueData: QueueData) => void;
}

export default function QueueButton({
  loket,
  title,
  subtitle,
  onQueueGenerated,
}: QueueButtonProps) {
  const [isLoading, setIsLoading] = useState(false);

  const handleGenerateQueue = async () => {
    setIsLoading(true);
    try {
      console.log("Creating queue for loket:", loket);

      // Check if we're in Tauri environment
      if ((window as any).__TAURI__) {
        console.log("Running in Tauri environment");
        const queueData = await tauriAPI.createQueue({
          loket_type: loket,
          patient_type: subtitle,
        });
        console.log("Queue created successfully:", queueData);
        onQueueGenerated(queueData);
      } else {
        console.log("Not in Tauri environment, using mock data");
        // Fallback for development - get next queue number from localStorage
        const storageKey = `queue_count_${loket}`;
        const currentCount = parseInt(
          localStorage.getItem(storageKey) || "0",
          10
        );
        const nextNumber = currentCount + 1;
        localStorage.setItem(storageKey, nextNumber.toString());

        const mockQueueData = {
          id: Date.now(),
          loket_type: loket,
          queue_number: nextNumber,
          queue_code: `${loket}${String(nextNumber).padStart(3, "0")}`,
          patient_type: subtitle,
          status: "waiting",
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        };
        console.log("Mock queue created:", mockQueueData);
        onQueueGenerated(mockQueueData);
      }
    } catch (error) {
      console.error("Error generating queue:", error);
      console.error("Error details:", JSON.stringify(error, null, 2));
      alert(
        `Terjadi kesalahan saat membuat antrian: ${error}. Silakan coba lagi.`
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <button
      onClick={handleGenerateQueue}
      disabled={isLoading}
      className="w-full max-w-md bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 
                 text-white font-bold py-8 px-12 rounded-xl shadow-lg hover:shadow-xl transform hover:scale-105 
                 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
    >
      <div className="text-center">
        <div className="text-2xl mb-2">LOKET {loket}</div>
        <div className="text-lg font-semibold text-yellow-200">{title}</div>
        {subtitle && <div className="text-sm mt-1 opacity-90">{subtitle}</div>}
        {isLoading && (
          <div className="mt-3 flex items-center justify-center">
            <Printer className="w-5 h-5 animate-spin mr-2" />
            Membuat antrian...
          </div>
        )}
      </div>
    </button>
  );
}
