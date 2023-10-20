import init, { greet, setup, Gesturizer } from "gresture";

window.laszlo = function laszlo(x,y) {
	alert(x)
}

init().then(() => {
	setup()

	const g = new Gesturizer()
	const svg = document.querySelector('svg')
	const cam = svg.querySelector('g')

	
	document.addEventListener('mousemove', (evt) => {
		g.mouse_move(evt.clientX - window.innerWidth / 2, evt.clientY  - window.innerHeight / 2)
	})

	svg.addEventListener('mousedown', (evt) => {
		g.mouse_down()
	})

	document.addEventListener('mouseup', (evt) => {
		g.mouse_up()
	})

	function tick() {
		const x = g.get_mouse().get_x()
		const y = g.get_mouse().get_y()

		cam.setAttribute('transform', `translate(${x}, ${y})`)

		requestAnimationFrame(tick)
	}

	tick()
})