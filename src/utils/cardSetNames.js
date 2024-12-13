const getCardSetDisplayName = (cardSet) => {
  switch (cardSet) {
    case 'Base':
      return 'Base Game';
    case 'SenseiPath':
      return "Sensei's Path";
    case 'PromotionalPack':
      return 'Promotional Cards';
    case 'WayOfTheWind':
      return 'Way of the Wind';
    default:
      return cardSet;
  }
};

export default getCardSetDisplayName;
