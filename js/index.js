import CONFIG from './config.js';
import View from './view.js';
import Storage from './storage.js';

import("../pkg/index.js").then(module => {
    let Game = module.Game;
    let Vector = module.Vector;
    let Movement = module.Movement;

    const MOVEMENT_KEYS = {
        [Movement.UP]: [87, 38], // 'W' and 'Up Arrow'
        [Movement.DOWN]: [83, 40], // 'S' and 'Down Arrow'
        [Movement.LEFT]: [65, 37], // 'A' and 'Left Arrow'
        [Movement.RIGHT]: [68, 39], // 'D' and 'Right Arrow'
    }
    const STOP_KEY = 32; // Space key

    class Controller {
        constructor(onStop=() => {}) {
            window.addEventListener('keydown', ({which}) => {
                console.log(which);
                this.movement = Object.keys(MOVEMENT_KEYS)
                    .find(key => MOVEMENT_KEYS[key].includes(which));
                this.movement = parseInt(this.movement);
            });
            window.addEventListener('keyup', ({which}) => {
                this.movement = undefined;
                if (which === STOP_KEY) {
                    onStop();
                }
            })
        }
    }

    class GameManager {
        constructor() {
            this.restart();
            this.view = new View(
                this.game.width,
                this.game.height,
                this.render.bind(this)
            );
            this.controller = new Controller(this.onStop.bind(this));
        }
    
        restart() {
            this.game = new Game(
                CONFIG.WIDTH,
                CONFIG.HEIGHT,
                CONFIG.SPEED,
                CONFIG.SNAKE_LENGTH,
                new Vector(CONFIG.SNAKE_DIRECTION_X, CONFIG.SNAKE_DIRECTION_Y)
            );
            console.log(this.game);
        }

        onStop() {
            const now = Date.now();
            if (this.stopTime) {
                this.stopTime = undefined;
                this.lastUpdate = this.time + now - this.lastUpdate;
            } else {
                this.stopTime = now;
            }
        }

        render() {
            this.view.render(
                this.game.food,
                this.game.get_snake(),
                this.game.score,
                Storage.getBestScore(),
            )
        }

        tick() {
            if (!this.stopTime) {
                const lastUpdate = Date.now();
                if (this.lastUpdate) {
                    this.game.process(lastUpdate - this.lastUpdate, this.controller.movement);
                    if (this.game.score > Storage.getBestScore()) {
                        Storage.setBestScore(this.game.score);
                    }
                }
                this.lastUpdate = lastUpdate;
                this.render();
            }
        }

        run() {
            console.log('Running! 🚀');
            setInterval(this.tick.bind(this), 1000 / CONFIG.FPS);
        }
    }

    let game = new GameManager();

    game.run()
}).catch(console.error);
