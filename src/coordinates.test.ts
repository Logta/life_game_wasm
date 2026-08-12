import { describe, expect, test } from "bun:test";
import { getClickCoordinates, isValidCoordinate } from "./coordinates";

/**
 * getBoundingClientRect() の最小限のモックを持つCanvas要素を作成
 * Create a minimal canvas-like object with a stubbed getBoundingClientRect()
 */
function createCanvasStub(rect: Partial<DOMRect>): HTMLCanvasElement {
  return {
    getBoundingClientRect: () => rect as DOMRect,
  } as unknown as HTMLCanvasElement;
}

function createMouseEvent(clientX: number, clientY: number): MouseEvent {
  return { clientX, clientY } as MouseEvent;
}

describe("getClickCoordinates", () => {
  test("キャンバス左上を基準にセルサイズで割った座標を返す", () => {
    const canvas = createCanvasStub({ left: 0, top: 0 });
    const event = createMouseEvent(25, 35);
    expect(getClickCoordinates(canvas, event, 10)).toEqual({ row: 3, col: 2 });
  });

  test("キャンバスのオフセット(left/top)を考慮する", () => {
    const canvas = createCanvasStub({ left: 100, top: 50 });
    const event = createMouseEvent(120, 65);
    expect(getClickCoordinates(canvas, event, 10)).toEqual({ row: 1, col: 2 });
  });

  test("セルの境界上はそのセルの座標として切り下げる", () => {
    const canvas = createCanvasStub({ left: 0, top: 0 });
    const event = createMouseEvent(20, 10);
    expect(getClickCoordinates(canvas, event, 10)).toEqual({ row: 1, col: 2 });
  });
});

describe("isValidCoordinate", () => {
  test("フィールド範囲内の座標はtrue", () => {
    expect(isValidCoordinate({ row: 0, col: 0 }, 10, 10)).toBe(true);
    expect(isValidCoordinate({ row: 9, col: 9 }, 10, 10)).toBe(true);
  });

  test("負の座標はfalse", () => {
    expect(isValidCoordinate({ row: -1, col: 0 }, 10, 10)).toBe(false);
    expect(isValidCoordinate({ row: 0, col: -1 }, 10, 10)).toBe(false);
  });

  test("フィールドの幅・高さと同じ値(範囲外)はfalse", () => {
    expect(isValidCoordinate({ row: 10, col: 0 }, 10, 10)).toBe(false);
    expect(isValidCoordinate({ row: 0, col: 10 }, 10, 10)).toBe(false);
  });
});
