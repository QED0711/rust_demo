class Point {
    constructor(x, y){
        this.x = x;
        this.y = y;
    }

    distance_to(p){
        return Math.sqrt((this.x - p.x)**2 + (this.y - p.y)**2)
    }
}

const run = () => {

    const points = Array.from({length: 25_000}, () => new Point(Math.random(), Math.random()));

    console.time("execution_time")
    points.map(p => {
        const dists = [];
        points.forEach(pInner => p !== pInner && dists.push(p.distance_to(pInner)));
        return dists;
    })
    console.timeEnd("execution_time");

}

run();