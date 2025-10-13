import { useState, useEffect } from "react";
import QueueButton from "@/react-app/components/QueueButton";
import QueueTicket from "@/react-app/components/QueueTicket";
import { tauriAPI, type QueueData } from "../utils/tauri-api";

export default function Home() {
  const [currentQueue, setCurrentQueue] = useState<QueueData | null>(null);
  const [queueCounts, setQueueCounts] = useState({ loketA: 0, loketB: 0 });

  useEffect(() => {
    // Load Google Fonts
    const link = document.createElement("link");
    link.href =
      "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap";
    link.rel = "stylesheet";
    document.head.appendChild(link);

    // Fetch current queue counts
    fetchQueueCounts();
  }, []);

  const fetchQueueCounts = async () => {
    try {
      if ((window as any).__TAURI__) {
        console.log("Fetching queue counts from Tauri");
        const data = await tauriAPI.getCurrentQueues();
        setQueueCounts({ loketA: data.loket_a, loketB: data.loket_b });
      } else {
        console.log(
          "Not in Tauri environment, using mock counts from localStorage"
        );
        // Get counts from localStorage for fallback
        const loketACount = parseInt(
          localStorage.getItem("queue_count_A") || "0",
          10
        );
        const loketBCount = parseInt(
          localStorage.getItem("queue_count_B") || "0",
          10
        );
        setQueueCounts({ loketA: loketACount, loketB: loketBCount });
      }
    } catch (error) {
      console.error("Error fetching queue counts:", error);
      // Set default values if API fails
      setQueueCounts({ loketA: 0, loketB: 0 });
    }
  };

  const handleQueueGenerated = (queueData: QueueData) => {
    console.log("Queue generated, showing modal:", queueData);
    setCurrentQueue(queueData);
    fetchQueueCounts(); // Refresh counts
  };

  const handleCloseTicket = () => {
    setCurrentQueue(null);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-orange-200 via-yellow-100 to-orange-300 print:bg-white">
      {/* Header with logos and title */}
      <div className="bg-gradient-to-r from-orange-300 to-yellow-200 py-8 px-4 print:hidden">
        <div className="max-w-4xl mx-auto">
          <div className="flex items-center justify-between mb-6">
            {/* Left logo - Purbalingga */}
            <div className="w-32 h-32 flex items-center justify-center">
              <img
                src="https://mocha-cdn.com/0199cbc3-5101-713f-b238-a674a5ab36da/logo_purbalingga.png"
                alt="Logo Purbalingga"
                className="w-32 h-32 object-contain"
              />
            </div>

            {/* Center title */}
            <div className="text-center flex-1 mx-8">
              <h1 className="text-3xl md:text-4xl font-bold text-gray-800 mb-2">
                SISTEM ANTRIAN
              </h1>
              <h2 className="text-2xl md:text-3xl font-bold text-gray-700 mb-1">
                PUSKESMAS MREBET
              </h2>
              <p className="text-lg text-gray-600">KABUPATEN PURBALINGGA</p>
            </div>

            {/* Right logo - Puskesmas */}
            <div className="w-32 h-32 flex items-center justify-center">
              <img
                src="https://mocha-cdn.com/0199cbc3-5101-713f-b238-a674a5ab36da/logo_puskesmas.png"
                alt="Logo Puskesmas"
                className="w-28 h-28 object-contain"
              />
            </div>
          </div>
        </div>
      </div>

      {/* Main content */}
      <div className="max-w-2xl mx-auto px-4 py-12 print:hidden">
        <div className="flex flex-col items-center space-y-8">
          <QueueButton
            loket="A"
            title="PASIEN BIASA"
            subtitle=""
            onQueueGenerated={handleQueueGenerated}
          />

          <QueueButton
            loket="B"
            title="BALITA IBU HAMIL & LANSIA"
            subtitle=""
            onQueueGenerated={handleQueueGenerated}
          />
        </div>

        {/* Queue status display */}
        <div className="mt-12 bg-white/50 backdrop-blur-sm rounded-xl p-6 shadow-lg">
          <h3 className="text-lg font-semibold text-gray-800 mb-4 text-center">
            Status Antrian Hari Ini
          </h3>
          <div className="grid grid-cols-2 gap-4">
            <div className="text-center p-4 bg-blue-100 rounded-lg">
              <div className="text-2xl font-bold text-blue-800">
                {queueCounts.loketA}
              </div>
              <div className="text-sm text-blue-600">Loket A</div>
            </div>
            <div className="text-center p-4 bg-blue-100 rounded-lg">
              <div className="text-2xl font-bold text-blue-800">
                {queueCounts.loketB}
              </div>
              <div className="text-sm text-blue-600">Loket B</div>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="mt-8 text-center text-gray-600">
          <p className="text-sm">PUSKESMAS MREBET KABUPATEN PURBALINGGA</p>
          <p className="text-xs mt-1 text-gray-500">by Garendy</p>
        </div>
      </div>

      {/* Queue ticket modal */}
      {currentQueue && (
        <QueueTicket queueData={currentQueue} onClose={handleCloseTicket} />
      )}
    </div>
  );
}
