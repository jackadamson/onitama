import { useState, useEffect } from 'react';

const DEFAULT_GAME_SETTINGS = {
  disabledCardSets: [],
  numberOfWindCards: null, // null for Random
  forceWindSpiritInclusion: false,
  enableLightAndShadow: true,
  forceLightAndShadow: false,
  lightAndShadowMode: null, // null for Random
};

const useGameSettings = () => {
  // Initialize settings from localStorage or use default values
  const [gameSettings, setGameSettings] = useState(() => {
    const storedSettings = localStorage.getItem('game_settings');
    return storedSettings ? JSON.parse(storedSettings) : DEFAULT_GAME_SETTINGS;
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
      lightAndShadowMode:
        updatedSettings.lightAndShadowMode === 'Random' ? null : updatedSettings.lightAndShadowMode,
    }));
  };

  return [gameSettings, updateGameSettings, DEFAULT_GAME_SETTINGS];
};

export { DEFAULT_GAME_SETTINGS };
export default useGameSettings;
