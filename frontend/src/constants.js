// Definido pelo servidor via game_info; 2000 é só fallback até a info chegar.
export let WORLD_SIZE = 2000;
export function setWorldSize(size) {
  WORLD_SIZE = size;
}
export const MINIMAP_SIZE = 150;
export const MINIMAP_PADDING = 20;
export const VIEW_RADIUS = 600;
