export function getBuildingImagePath(building) {
  const faction = building.faction ? `${building.faction}-` : '';
  const suit = building.suit ? `_${building.suit}` : '';

  return `/image/piece/token.${faction}${building.building}${suit}.png`;
}

export function getTokenImagePath(token) {
  const faction = token.faction ? `${token.faction}-` : '';
  const suit = token.suit ? `_${token.suit}` : '';

  return `/image/piece/token.${faction}${token.token}${suit}.png`;
}

export function getVictoryPointImagePath(faction) {
  return `/image/piece/victory-points/token.${faction}-vp.png`;
}

export function getMapImagePath(map) {
  return `/image/map/map-${map}.jpg`;
}

export function getFactionIconPath(faction) {
  return `/image/piece/icons/token.${faction}-icon.png`;
}

export function getItemImagePath(item, ruin) {
  const ruinPath = ruin ? 'ruin_' : '';

  return `/image/piece/item/token.item-${ruinPath}${item}.png`;
}

export function getSharedCardPath(card) {
  const path = card ? `front.${card.suit}-${card.card}` : 'back';

  return `/image/card/standard/card-shared-${path}.jpg`;
}

export function getEyrieLeaderPath(leader) {
  const path = leader ? `front.${leader}` : 'back';

  return `/image/card/eyrie/card-eyrie_leader-${path}.jpg`;
}

export function getVagabondCharacterPath(character) {
  const path = character ? `front.${character}` : 'back';

  return `/image/card/vagabond-characters/card-vagabond_character-${path}.jpg`;
}

export function getFactionBoardFrontPath(faction) {
  return `/image/board/board.${faction}-front.jpg`;
}

export function getFactionBoardBackPath(faction) {
  return `/image/board/board.${faction}-back.jpg`;
}

export function getOutcastImagePath(hated) {
  return `/image/piece/token.cult-${hated ? 'hated_' : ''}outcast.png`;
}

export function getMinisterCardPath(minister) {
  return minister
    ? `/image/card/ministers/card.minister-${minister}.png`
    : '/image/card/ministers/card.minister-back.png';
}

export function getCrownImage() {
  return '/image/crown.svg';
}
