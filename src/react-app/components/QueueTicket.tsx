import { useEffect } from "react";
import { type QueueData } from "../utils/tauri-api";

interface QueueTicketProps {
  queueData: QueueData;
  onClose: () => void;
}

export default function QueueTicket({ queueData, onClose }: QueueTicketProps) {
  console.log("QueueTicket rendered with data:", queueData);

  const formatDate = (timestamp: string) => {
    const date = new Date(timestamp);
    const day = String(date.getDate()).padStart(2, "0");
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const year = date.getFullYear();
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");
    const seconds = String(date.getSeconds()).padStart(2, "0");
    return `${day}/${month}/${year}, ${hours}.${minutes}.${seconds}`;
  };

  const handlePrint = () => {
    try {
      console.log("Manual print triggered");
      // Use setTimeout to ensure DOM is ready
      setTimeout(() => {
        window.print();
      }, 100);
    } catch (error) {
      console.error("Print error:", error);
      alert("Gagal membuka dialog print. Silakan gunakan Cmd+P atau Ctrl+P");
    }
  };

  useEffect(() => {
    // Only auto-print if NOT in Tauri environment (thermal printer fallback)
    // In Tauri, thermal printer already printed in QueueButton
    if (!(window as any).__TAURI__) {
      console.log("Not in Tauri, using browser print dialog");
      const timer = setTimeout(() => {
        try {
          console.log("Auto printing...");
          window.print();
        } catch (error) {
          console.error("Auto print error:", error);
        }
      }, 1000);
      return () => clearTimeout(timer);
    } else {
      console.log("In Tauri environment, thermal printer already handled");
    }
  }, []);

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 print:bg-white print:static print:inset-auto">
      <div className="bg-white p-8 rounded-lg shadow-xl max-w-md w-full mx-4 print:shadow-none print:rounded-none print:max-w-none print:mx-0">
        {/* Screen view controls */}
        <div className="flex justify-between items-center mb-6 print:hidden">
          <h2 className="text-xl font-bold">Tiket Antrian</h2>
          <div className="space-x-2">
            <button
              onClick={handlePrint}
              className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
            >
              Print
            </button>
            <button
              onClick={onClose}
              className="bg-gray-500 text-white px-4 py-2 rounded hover:bg-gray-600"
            >
              Tutup
            </button>
          </div>
        </div>

        {/* Ticket content */}
        <div className="text-center print:text-black print:font-mono print:w-full print:max-w-none print:bg-white">
          <div className="p-6 print:p-2 print:pb-4">
            <div className="mb-4 print:mb-2">
              <h3 className="text-lg font-bold tracking-wider print:text-sm print:font-bold">
                PUSKESMAS MREBET
              </h3>
              <p className="text-sm tracking-wide print:text-xs">
                KAB. PURBALINGGA
              </p>
            </div>

            <div className="my-4 print:my-2">
              <div className="border-t-2 border-dashed border-gray-800 print:border-gray-800 print:border-t"></div>
            </div>

            <div className="mb-4 print:mb-2">
              <p className="text-lg font-bold tracking-wider print:text-sm print:font-bold">
                NOMOR ANTRIAN
              </p>
            </div>

            <div className="mb-6 print:mb-3">
              <div className="text-8xl font-black text-gray-800 mb-3 tracking-wider print:text-6xl print:mb-2 print:font-black">
                {queueData.queue_code}
              </div>
              <p className="text-base font-semibold tracking-wide print:text-xs print:font-semibold print:leading-tight">
                Loket {queueData.loket_type} -{" "}
                {queueData.loket_type === "A"
                  ? "Pasien Umum"
                  : "Balita Ibu Hamil dan Lansia"}
              </p>
            </div>

            <div className="my-4 print:my-2">
              <div className="border-t-2 border-dashed border-gray-800 print:border-gray-800 print:border-t"></div>
            </div>

            <div className="text-sm text-gray-800 space-y-2 mb-4 print:text-xs print:mb-2 print:space-y-1 print:text-black">
              <p className="leading-relaxed print:leading-tight">
                Terima kasih atas kunjungan Anda.
                <br />
                Jaga selalu kesehatan Anda dan keluarga.
              </p>
            </div>

            <div className="text-sm text-gray-800 print:text-xs print:text-black">
              <p>{formatDate(queueData.created_at)}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
