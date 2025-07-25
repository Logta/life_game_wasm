import init, { GameOfLife, init as wasmInit } from "../pkg/life_game_wasm.js";

// 型定義
interface GameConfig {
    cellSize: number;
    width: number;
    height: number;
    defaultFps: number;
}

interface Coordinates {
    row: number;
    col: number;
}

/**
 * ライフゲームを管理するゲームコントローラークラス
 */
class GameController {
    private game: GameOfLife;
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private config: GameConfig;
    private isPlaying: boolean = false;
    private animationId: number | null = null;
    private fps: number;
    private lastTime: number = 0;

    // UI要素
    private playPauseBtn: HTMLButtonElement;
    private stepBtn: HTMLButtonElement;
    private clearBtn: HTMLButtonElement;
    private randomBtn: HTMLButtonElement;
    private speedSlider: HTMLInputElement;
    private speedValue: HTMLSpanElement;
    private generationCounter: HTMLElement;

    constructor(canvas: HTMLCanvasElement, config: GameConfig) {
        this.canvas = canvas;
        const ctx = canvas.getContext("2d");
        if (!ctx) {
            throw new Error("キャンバスから2Dコンテキストを取得できませんでした");
        }
        this.ctx = ctx;
        this.config = config;
        this.fps = config.defaultFps;

        // ゲームを初期化
        this.game = new GameOfLife(config.width, config.height);

        // UI要素を取得
        this.playPauseBtn = this.getElement<HTMLButtonElement>("play-pause-btn");
        this.stepBtn = this.getElement<HTMLButtonElement>("step-btn");
        this.clearBtn = this.getElement<HTMLButtonElement>("clear-btn");
        this.randomBtn = this.getElement<HTMLButtonElement>("random-btn");
        this.speedSlider = this.getElement<HTMLInputElement>("speed-slider");
        this.speedValue = this.getElement<HTMLSpanElement>("speed-value");
        this.generationCounter = this.getElement<HTMLElement>("generation-counter");

        this.setupEventListeners();
        this.render();
    }

    /**
     * 型チェック付きでIDによって要素を取得
     */
    private getElement<T extends HTMLElement>(id: string): T {
        const element = document.getElementById(id);
        if (!element) {
            throw new Error(`ID "${id}" の要素が見つかりません`);
        }
        return element as T;
    }

    /**
     * すべてのイベントリスナーを設定
     */
    private setupEventListeners(): void {
        // キャンバスクリックハンドラー
        this.canvas.addEventListener("click", this.handleCanvasClick.bind(this));

        // ボタンハンドラー
        this.playPauseBtn.addEventListener("click", this.togglePlayPause.bind(this));
        this.stepBtn.addEventListener("click", this.step.bind(this));
        this.clearBtn.addEventListener("click", this.clear.bind(this));
        this.randomBtn.addEventListener("click", this.randomize.bind(this));

        // 速度スライダーハンドラー
        this.speedSlider.addEventListener("input", this.handleSpeedChange.bind(this));

        // 速度表示を初期化
        this.updateSpeedDisplay();
    }

    /**
     * キャンバスクリックイベントを処理
     */
    private handleCanvasClick(event: MouseEvent): void {
        const coords = this.getClickCoordinates(event);
        if (this.isValidCoordinate(coords)) {
            this.game.toggle_cell(coords.row, coords.col);
            this.render();
        }
    }

    /**
     * ゲームグリッドに対するクリック座標を取得
     */
    private getClickCoordinates(event: MouseEvent): Coordinates {
        const rect = this.canvas.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        const col = Math.floor(x / this.config.cellSize);
        const row = Math.floor(y / this.config.cellSize);
        return { row, col };
    }

    /**
     * 座標が有効かチェック
     */
    private isValidCoordinate(coords: Coordinates): boolean {
        return coords.row >= 0 && 
               coords.row < this.config.height && 
               coords.col >= 0 && 
               coords.col < this.config.width;
    }

    /**
     * FPS制御付きのメインゲームループ
     */
    private gameLoop(currentTime: number = 0): void {
        if (!this.isPlaying) return;

        const deltaTime = currentTime - this.lastTime;
        const targetDeltaTime = 1000 / this.fps;

        if (deltaTime >= targetDeltaTime) {
            this.tick();
            this.lastTime = currentTime - (deltaTime % targetDeltaTime);
        }

        this.animationId = requestAnimationFrame(this.gameLoop.bind(this));
    }

    /**
     * ゲームを1ティック進める
     */
    private tick(): void {
        this.game.tick();
        this.render();
    }

    /**
     * ゲーム状態をレンダリング
     */
    private render(): void {
        this.game.render(this.ctx);
        this.updateGenerationCounter();
    }

    /**
     * 世代カウンターの表示を更新
     */
    private updateGenerationCounter(): void {
        this.generationCounter.textContent = `世代: ${this.game.generation()}`;
    }

    /**
     * ゲームを開始
     */
    private play(): void {
        this.isPlaying = true;
        this.playPauseBtn.textContent = "一時停止";
        this.stepBtn.disabled = true;
        this.lastTime = performance.now();
        this.gameLoop();
    }

    /**
     * ゲームを一時停止
     */
    private pause(): void {
        this.isPlaying = false;
        this.playPauseBtn.textContent = "再生";
        this.stepBtn.disabled = false;
        if (this.animationId !== null) {
            cancelAnimationFrame(this.animationId);
            this.animationId = null;
        }
    }

    /**
     * 再生/一時停止状態を切り替え
     */
    private togglePlayPause(): void {
        if (this.isPlaying) {
            this.pause();
        } else {
            this.play();
        }
    }

    /**
     * ゲームを1ステップ進める（一時停止時）
     */
    private step(): void {
        if (!this.isPlaying) {
            this.tick();
        }
    }

    /**
     * ゲームフィールドをクリア
     */
    private clear(): void {
        if (this.isPlaying) {
            this.pause();
        }
        this.game.clear();
        this.render();
    }

    /**
     * ゲームフィールドをランダム化
     */
    private randomize(): void {
        if (this.isPlaying) {
            this.pause();
        }
        this.game.randomize();
        this.render();
    }

    /**
     * 速度スライダーの変更を処理
     */
    private handleSpeedChange(event: Event): void {
        const target = event.target as HTMLInputElement;
        this.fps = parseInt(target.value, 10);
        this.updateSpeedDisplay();
    }

    /**
     * 速度表示を更新
     */
    private updateSpeedDisplay(): void {
        this.speedValue.textContent = `${this.fps} FPS`;
    }

    /**
     * リソースをクリーンアップ
     */
    public destroy(): void {
        this.pause();
        this.game.free();
    }
}

/**
 * ライフゲームを初期化して実行
 */
async function main(): Promise<void> {
    try {
        // WASMモジュールを初期化
        await init();
        
        // カスタム初期化関数を呼び出し
        wasmInit();

        // キャンバス要素を取得
        const canvas = document.getElementById("life-canvas");
        if (!(canvas instanceof HTMLCanvasElement)) {
            throw new Error("キャンバス要素が見つからないか、キャンバスではありません");
        }

        // 設定
        const config: GameConfig = {
            cellSize: 10,
            width: Math.floor(canvas.width / 10),
            height: Math.floor(canvas.height / 10),
            defaultFps: 10
        };

        // ゲームコントローラーを作成して開始
        const controller = new GameController(canvas, config);

        // ページアンロード時のクリーンアップを処理
        window.addEventListener("beforeunload", () => {
            controller.destroy();
        });

    } catch (error) {
        console.error("ライフゲームの初期化に失敗しました:", error);
        // ユーザーにエラーメッセージを表示
        const errorElement = document.createElement("div");
        errorElement.style.cssText = "color: red; padding: 20px; text-align: center;";
        errorElement.textContent = `初期化に失敗しました: ${error instanceof Error ? error.message : String(error)}`;
        document.body.prepend(errorElement);
    }
}

// アプリケーションを開始
main();