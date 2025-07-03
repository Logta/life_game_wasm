import init, { GameOfLife, init as wasmInit } from "../pkg/life_game_wasm.js";

// Types
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
 * Game controller class that manages the Game of Life
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

    // UI Elements
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
            throw new Error("Failed to get 2D context from canvas");
        }
        this.ctx = ctx;
        this.config = config;
        this.fps = config.defaultFps;

        // Initialize game
        this.game = new GameOfLife(config.width, config.height);

        // Get UI elements
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
     * Get an element by ID with type checking
     * 型チェック付きでIDによって要素を取得
     */
    private getElement<T extends HTMLElement>(id: string): T {
        const element = document.getElementById(id);
        if (!element) {
            throw new Error(`Element with id "${id}" not found`);
        }
        return element as T;
    }

    /**
     * Set up all event listeners
     * すべてのイベントリスナーを設定
     */
    private setupEventListeners(): void {
        // Canvas click handler
        this.canvas.addEventListener("click", this.handleCanvasClick.bind(this));

        // Button handlers
        this.playPauseBtn.addEventListener("click", this.togglePlayPause.bind(this));
        this.stepBtn.addEventListener("click", this.step.bind(this));
        this.clearBtn.addEventListener("click", this.clear.bind(this));
        this.randomBtn.addEventListener("click", this.randomize.bind(this));

        // Speed slider handler
        this.speedSlider.addEventListener("input", this.handleSpeedChange.bind(this));

        // Initialize speed display
        this.updateSpeedDisplay();
    }

    /**
     * Handle canvas click events
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
     * Get click coordinates relative to the game grid
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
     * Check if coordinates are valid
     * 座標が有効かチェック
     */
    private isValidCoordinate(coords: Coordinates): boolean {
        return coords.row >= 0 && 
               coords.row < this.config.height && 
               coords.col >= 0 && 
               coords.col < this.config.width;
    }

    /**
     * Main game loop with FPS control
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
     * Advance the game by one tick
     * ゲームを1ティック進める
     */
    private tick(): void {
        this.game.tick();
        this.render();
    }

    /**
     * Render the game state
     * ゲーム状態をレンダリング
     */
    private render(): void {
        this.game.render(this.ctx);
        this.updateGenerationCounter();
    }

    /**
     * Update the generation counter display
     * 世代カウンターの表示を更新
     */
    private updateGenerationCounter(): void {
        this.generationCounter.textContent = `Generation: ${this.game.generation()}`;
    }

    /**
     * Start playing the game
     * ゲームを開始
     */
    private play(): void {
        this.isPlaying = true;
        this.playPauseBtn.textContent = "Pause";
        this.stepBtn.disabled = true;
        this.lastTime = performance.now();
        this.gameLoop();
    }

    /**
     * Pause the game
     * ゲームを一時停止
     */
    private pause(): void {
        this.isPlaying = false;
        this.playPauseBtn.textContent = "Play";
        this.stepBtn.disabled = false;
        if (this.animationId !== null) {
            cancelAnimationFrame(this.animationId);
            this.animationId = null;
        }
    }

    /**
     * Toggle play/pause state
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
     * Advance the game by one step (when paused)
     * ゲームを1ステップ進める（一時停止時）
     */
    private step(): void {
        if (!this.isPlaying) {
            this.tick();
        }
    }

    /**
     * Clear the game field
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
     * Randomize the game field
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
     * Handle speed slider change
     * 速度スライダーの変更を処理
     */
    private handleSpeedChange(event: Event): void {
        const target = event.target as HTMLInputElement;
        this.fps = parseInt(target.value, 10);
        this.updateSpeedDisplay();
    }

    /**
     * Update the speed display
     * 速度表示を更新
     */
    private updateSpeedDisplay(): void {
        this.speedValue.textContent = `${this.fps} FPS`;
    }

    /**
     * Clean up resources
     * リソースをクリーンアップ
     */
    public destroy(): void {
        this.pause();
        this.game.free();
    }
}

/**
 * Initialize and run the Game of Life
 * ライフゲームを初期化して実行
 */
async function main(): Promise<void> {
    try {
        // Initialize WASM module
        await init();
        
        // Call custom init function
        wasmInit();

        // Get canvas element
        const canvas = document.getElementById("life-canvas");
        if (!(canvas instanceof HTMLCanvasElement)) {
            throw new Error("Canvas element not found or is not a canvas");
        }

        // Configuration
        const config: GameConfig = {
            cellSize: 10,
            width: Math.floor(canvas.width / 10),
            height: Math.floor(canvas.height / 10),
            defaultFps: 10
        };

        // Create and start the game controller
        const controller = new GameController(canvas, config);

        // Handle cleanup on page unload
        window.addEventListener("beforeunload", () => {
            controller.destroy();
        });

    } catch (error) {
        console.error("Failed to initialize Game of Life:", error);
        // Display error message to user
        const errorElement = document.createElement("div");
        errorElement.style.cssText = "color: red; padding: 20px; text-align: center;";
        errorElement.textContent = `Failed to initialize: ${error instanceof Error ? error.message : String(error)}`;
        document.body.prepend(errorElement);
    }
}

// Start the application
main();