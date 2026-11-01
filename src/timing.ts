/**
 * フレーム進行の判定結果
 * Result of a frame-step decision
 */
export interface FrameStep {
  /** この描画フレームで世代を進めるかどうか */
  advance: boolean;
  /** 次回の判定に使う基準時刻 */
  lastTime: number;
}

/**
 * requestAnimationFrame の時刻と目標 FPS から、世代を進めるかどうかを決める（純粋関数）
 * Decide whether to advance a generation for this animation frame (pure function)
 *
 * 進める場合は、目標間隔の端数を次回へ持ち越すように基準時刻を補正する。
 */
export function computeFrameStep(currentTime: number, lastTime: number, fps: number): FrameStep {
  const deltaTime = currentTime - lastTime;
  const targetDeltaTime = 1000 / fps;
  if (deltaTime >= targetDeltaTime) {
    return { advance: true, lastTime: currentTime - (deltaTime % targetDeltaTime) };
  }
  return { advance: false, lastTime };
}

/**
 * 速度スライダーの値を FPS に変換する（純粋関数）
 * Parse the speed slider value into an FPS within [min, max] (pure function)
 *
 * 数値でない値は fallback を返し、範囲外の値は min/max に丸める。
 */
export function parseFps(value: string, fallback: number, min = 1, max = 60): number {
  const parsed = Number.parseInt(value, 10);
  if (Number.isNaN(parsed)) {
    return fallback;
  }
  return Math.min(max, Math.max(min, parsed));
}
