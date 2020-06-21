const getRange = length => [...Array(length).keys()]

export default class View {
    constructor(gameWidth, gameHeight, onViewChange = () => {}) {
        this.gameWidth = gameWidth;
        this.gameHeight = gameHeight;
        this.container = document.getElementById('container');
        this.onViewChange = onViewChange;
        this.setUp();

        window.addEventListener('resize', () => {
            const [child] = this.container.children;
            if (child) {
                this.container.removeChild(child);
            }
            this.setUp();
            this.onViewChange();
        })
    }

    setUp() {
        const { width, height } = this.container.getBoundingClientRect();
        this.unitOnScreen = Math.min(
            width / this.gameWidth,
            height / this.gameHeight
        );
        this.projectDictance = distance => distance * this.unitOnScreen;
        this.projectPosition = position => position.scale_by(this.unitOnScreen);

        const canvas = document.createElement('canvas');
        this.container.appendChild(canvas);
        this.context = canvas.getContext('2d');

        canvas.setAttribute('width', this.projectDictance(this.gameWidth));
        canvas.setAttribute('height', this.projectDictance(this.gameHeight));
    }

    render(food, snake, score, bestScore) {
        this.context.clearRect(
            0,
            0,
            this.context.canvas.width,
            this.context.canvas.height,
        );

        this.context.globalAlpha = 0.2;
        this.context.fillStyle = 'black';

        getRange(this.gameWidth).forEach(column => {
            
            getRange(this.gameHeight)
                .filter(row => (column + row) % 2 === 1)
                .forEach(row => {
                    this.context.fillRect(
                        this.projectDictance(column),
                        this.projectDictance(row),
                        this.projectDictance(1),
                        this.projectDictance(1),
                    )
                });
        });

        this.context.globalAlpha = 1.0;

        this.context.font = `${this.unitOnScreen}px serif`;
        this.context.textAlign = 'center';
        this.context.textBaseline = 'middle';
        let projected_food = this.projectPosition(food);
        this.context.fillText(
            '🌮',
            projected_food.x,
            projected_food.y
        )

        this.context.lineWidth = this.unitOnScreen;
        this.context.strokeStyle = '#3498db';
        this.context.beginPath()
        snake
            .map(this.projectPosition)
            .forEach(({x, y}) => {
                this.context.lineTo(x, y);
            });
        this.context.stroke();

        document.getElementById('current-score').innerText = score;
        document.getElementById('high-score').innerHTML = bestScore;
    }
}