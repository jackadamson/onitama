import { useState, useEffect } from 'react';

const useGameSettings = () => {
  // Initialize settings from localStorage or use default values
  const [gameSettings, setGameSettings] = useState(() => {
    const storedSettings = localStorage.getItem('game_settings');
    return storedSettings
      ? JSON.parse(storedSettings)
      : {
          disabledCardSets: ['WayOfTheWind'],
          numberOfWindCards: 2, // Default is 2, not null
          forceWindSpiritInclusion: false,
        };
  });

  // Update settings in local storage whenever they change
  useEffect(() => {
    localStorage.setItem('game_settings', JSON.stringify(gameSettings));
  }, [gameSettings]);

  // Function to update game settings
  const updateGameSettings = (updatedSettings) => {
    setGameSettings((prevSettings) => ({
      ...prevSettings,
      ...updatedSettings,
      // Convert "Random" to null if needed
      numberOfWindCards:
        updatedSettings.numberOfWindCards === 'Random' ? null : updatedSettings.numberOfWindCards,
    }));
  };

  return [gameSettings, updateGameSettings];
};

export default useGameSettings;
