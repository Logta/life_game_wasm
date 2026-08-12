/**
 * グリッド座標
 * Grid coordinates
 */
export interface Coordinates {
  row: number;
  col: number;
}

/**
 * キャンバス上のクリック位置からグリッド座標を計算する（純粋関数）
 * Calculate grid coordinates from a canvas click position (pure function)
 */
export function getClickCoordinates(
  canvas: HTMLCanvasElement,
  event: MouseEvent,
  cellSize: number
): Coordinates {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  const col = Math.floor(x / cellSize);
  const row = Math.floor(y / cellSize);
  return { row, col };
}

/**
 * 座標がフィールドの範囲内かどうかを判定する（純粋関数）
 * Check whether coordinates are within the field bounds (pure function)
 */
export function isValidCoordinate(coords: Coordinates, width: number, height: number): boolean {
  return coords.row >= 0 && coords.row < height && coords.col >= 0 && coords.col < width;
}
