import { useState, useEffect } from 'react';
import { Play, HelpCircle } from 'lucide-react';
import { startTour, hasCompletedTour } from '../services/tourService';

export const TourButton = () => {
  const [hasSeenTour, setHasSeenTour] = useState(false);

  useEffect(() => {
    setHasSeenTour(hasCompletedTour());
  }, []);

  const handleStartTour = () => {
    startTour();
  };

  return (
    <button
      onClick={handleStartTour}
      className={`fixed bottom-6 right-6 z-50 flex items-center gap-2 px-4 py-3 rounded-full shadow-lg transition-all duration-300 hover:scale-105 ${
        hasSeenTour 
          ? 'bg-blue-600 text-white hover:bg-blue-700' 
          : 'bg-gradient-to-r from-pink-500 to-purple-600 text-white hover:from-pink-600 hover:to-purple-700'
      }`}
      title="Take a tour with Sonar!"
    >
      {hasSeenTour ? (
        <>
          <Play className="h-5 w-5" />
          <span className="font-medium">Tour Again</span>
        </>
      ) : (
        <>
          <HelpCircle className="h-5 w-5" />
          <span className="font-medium">Meet Sonar!</span>
        </>
      )}
    </button>
  );
};
